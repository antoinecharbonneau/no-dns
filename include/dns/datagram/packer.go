package datagram

import "errors"

func unpackDnsHeaderFlags(flags uint16) DnsHeaderFlags {
	return DnsHeaderFlags{
		(flags & 0x8000) != 0,
		uint8(flags >> 11 & 0x000F),
		(flags & 0x0400) != 0,
		(flags & 0x0200) != 0,
		(flags & 0x0100) != 0,
		(flags & 0x0080) != 0,
		uint8(flags >> 4 & 0x0007),
		uint8(flags & 0x000F),
	}
}

func packDnsHeaderFlags(flags DnsHeaderFlags) uint16 {
	var result uint16 = 0

	if flags.QR {
		result |= 0x8000
	}

	result |= uint16(flags.OPCODE) << 11

	if flags.AA {
		result |= 0x0400
	}

	if flags.TC {
		result |= 0x0200
	}

	if flags.RD {
		result |= 0x0100
	}

	if flags.RA {
		result |= 0x0080
	}

	result |= uint16(flags.Z) << 4
	result |= uint16(flags.RCODE)

	return result
}

func unpackDnsHeader(stream [12]byte) DnsHeader {
	return DnsHeader{
		uint16(stream[0])<<8 | uint16(stream[1]),
		unpackDnsHeaderFlags(uint16(stream[2])<<8 | uint16(stream[3])),
		uint16(stream[4])<<8 | uint16(stream[5]),
		uint16(stream[6])<<8 | uint16(stream[7]),
		uint16(stream[8])<<8 | uint16(stream[9]),
		uint16(stream[10])<<8 | uint16(stream[11]),
	}
}

func packDnsHeader(header DnsHeader) [12]byte {
	flags := packDnsHeaderFlags(header.flags)
	return [12]byte{
		byte(header.ID >> 8),
		byte(header.ID),
		byte(flags >> 8),
		byte(flags),
		byte(header.QDCOUNT >> 8),
		byte(header.QDCOUNT),
		byte(header.ANCOUNT >> 8),
		byte(header.ANCOUNT),
		byte(header.NSCOUNT >> 8),
		byte(header.NSCOUNT),
		byte(header.ARCOUNT >> 8),
		byte(header.ARCOUNT),
	}
}

func unpackDnsNameField(stream []byte, offset uint16) (string, uint16, error) {
	// TODO: Figure out the 255 character length
	// TODO: Make sure to respect the rules for a string
	// TODO: Implement reference ?
	var index uint16 = offset
	var formattedName string = ""
	var i uint8
	for stream[index] != 0 {
		if stream[index]&0xC0 == 0xC0 {
			// Earlier reference
			var pointer uint16 = (uint16(stream[index])&0x3F)<<8 | (uint16(stream[index+1]))
			referenced, _, err := unpackDnsNameField(stream, pointer)
			if err != nil {
				return "", 0, err
			}
			formattedName += referenced + "."
			index += 1
			break
		}
		sequenceLength := stream[index]
		index++
		for i = 0; i < sequenceLength; i++ {
			formattedName += string(stream[index+uint16(i)])
		}
		formattedName += "."
		index += uint16(i)
		if len(formattedName) >= 255 {
			return "", 0, &NameTooLong{}
		}
		if index >= uint16(len(stream)) {
			return "", 0, &NameNoEnd{}
		}
	}
	formattedName = formattedName[:len(formattedName)-1]

	return formattedName, index + 1, nil
}

func unpackDnsQuestion(question []byte, offset uint16) (DnsQuestion, uint16, error) {
	QNAME, index, err := unpackDnsNameField(question, offset)

	if err != nil {
		return DnsQuestion{}, 0, err
	}

	return DnsQuestion{
		QNAME,
		(uint16(question[index])<<8 | uint16(question[index+1])),
		(uint16(question[index+2])<<8 | uint16(question[index+3])),
	}, index + 4, nil
}

func packDnsQuestion(question DnsQuestion) []byte {
	var result []byte
	result = append(result, question.QNAME...)
	result = append(result, uint8(question.QTYPE>>8), uint8(question.QTYPE))
	result = append(result, uint8(question.QCLASS>>8), uint8(question.QCLASS))
	return result
}

func unpackDnsResourceRecord(stream []byte, offset uint16) (DnsResourceRecord, uint16, error) {
	NAME, index, err := unpackDnsNameField(stream, offset)
	if err != nil {
		return DnsResourceRecord{}, 0, err
	}

	RDLENGTH := uint16(stream[index+8])<<8 | uint16(stream[index+9])
	TYPE := uint16(stream[index])<<8 | uint16(stream[index+1])
	RDATA, _, err := unpackRDATA(stream, index+10, RDLENGTH, TYPE)

	return DnsResourceRecord{
		NAME,
		TYPE,
		uint16(stream[index+2])<<8 | uint16(stream[index+3]),
		int32(stream[index+4])<<24 | int32(stream[index+5])<<16 | int32(stream[index+6])<<8 | int32(stream[index+7]),
		RDLENGTH,
		RDATA,
	}, RDLENGTH + index + 10, nil
}

func packDnsResourceRecord(resource DnsResourceRecord) []byte {
	var result []byte
	result = append(result, resource.NAME...)
	result = append(result, uint8(resource.TYPE>>8), uint8(resource.TYPE))
	result = append(result, uint8(resource.CLASS>>8), uint8(resource.CLASS))
	result = append(result, uint8(resource.TTL>>24), uint8(resource.TTL>>16), uint8(resource.TTL>>8), uint8(resource.TTL))
	result = append(result, uint8(resource.RDLENGTH>>8), uint8(resource.RDLENGTH))
	result = append(result, packRDATA(resource.RDATA)...)

	return result
}

func unpackDnsResourceRecords(datagram []byte, count uint16, offset uint16) ([]DnsResourceRecord, uint16, error) {
	var resourceRecords []DnsResourceRecord
	var i uint16
	for i = 0; i < count; i++ {
		resourceRecord, readByte, err := unpackDnsResourceRecord(datagram, offset)
		if err != nil {
			return nil, 0, err
		}
		offset += readByte
		resourceRecords = append(resourceRecords, resourceRecord)
	}
	return resourceRecords, offset, nil
}

func errorDatagram(header DnsHeader, err error) DnsDatagram {
	if errors.As(err, &NameTooLong{}) {
		header.flags.RCODE = 1
	} else {
		header.flags.RCODE = 2
	}
	header.QDCOUNT = 0
	header.ANCOUNT = 0
	header.NSCOUNT = 0
	header.ARCOUNT = 0
	return DnsDatagram{header, nil, nil, nil, nil}
}

func UnpackDnsDatagram(stream []byte) (DnsDatagram, error) {
	var headerByte [12]byte
	copy(headerByte[:], stream[0:12])
	var header DnsHeader = unpackDnsHeader(headerByte)
	var readHead uint16 = 12
	var i uint16 = 0

	var questions []DnsQuestion
	for i = 0; i < header.QDCOUNT; i++ {
		question, newReadHead, err := unpackDnsQuestion(stream, readHead)
		readHead = newReadHead
		if err != nil {
			return errorDatagram(header, err), err
		}
		questions = append(questions, question)
	}

	answers, readHead, err := unpackDnsResourceRecords(stream, header.ANCOUNT, readHead)
	if err != nil {
		return errorDatagram(header, err), err
	}

	authorities, readHead, err := unpackDnsResourceRecords(stream, header.NSCOUNT, readHead)
	if err != nil {
		return errorDatagram(header, err), err
	}

	additionals, _, err := unpackDnsResourceRecords(stream, header.ARCOUNT, readHead)
	if err != nil {
		return errorDatagram(header, err), err
	}

	return DnsDatagram{
		header,
		questions,
		answers,
		authorities,
		additionals,
	}, nil
}
