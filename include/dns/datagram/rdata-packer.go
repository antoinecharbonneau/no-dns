package datagram

func packRDATA(rd RDATA) []byte {
	var output []byte
	switch rd.rdata_type {
	case 1:
		output = append(output, packA_RDATA(rd.A)...)
	default:
		output = append(output, rd.other.data...)
	}
	return output
}

func unpackRDATA(stream []byte, offset uint16, length uint16, TYPE uint16) (RDATA, uint16, error) {
	var rdata RDATA
	rdata.rdata_type = TYPE
	readHead := offset
	switch TYPE {
	case 1:
		rdata.A = unpackA_RDATA(stream, offset)
		readHead += 4
	default:
		rdata.other.data = stream[offset : offset+length]
		readHead += length
	}
	return rdata, readHead, nil
}

func unpackA_RDATA(stream []byte, offset uint16) A_RDATA {
	var A A_RDATA
	A.address[0] = stream[offset]
	A.address[1] = stream[offset+1]
	A.address[2] = stream[offset+2]
	A.address[3] = stream[offset+3]
	return A
}

func packA_RDATA(A A_RDATA) []byte {
	return []byte{A.address[0], A.address[1], A.address[2], A.address[3]}
}
