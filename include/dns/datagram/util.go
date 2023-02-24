package datagram

import "fmt"

func PrintHeaderFlags(flags DnsHeaderFlags) {
	var QR string
	if flags.QR {
		QR = "Response"
	} else {
		QR = "Query"
	}
	fmt.Printf("QR: %v\n", QR)

	var OPCODE string
	switch flags.OPCODE {
	case 0:
		OPCODE = "Query"
	case 1:
		OPCODE = "Inverse Query"
	case 2:
		OPCODE = "Status"
	default:
		OPCODE = "Future Use / Not Implemented"
	}
	fmt.Printf("OPCODE: %v\n", OPCODE)

	var AA string
	if flags.AA {
		AA = "Authoritative"
	} else {
		AA = "Non-authoritative"
	}
	fmt.Printf("AA: %v\n", AA)

	var TC string
	if flags.TC {
		TC = "Truncated"
	} else {
		TC = "Non-truncated"
	}
	fmt.Printf("TC: %v\n", TC)

	var RD string
	if flags.RD {
		RD = "Recursion desired"
	} else {
		RD = "No recursion desired"
	}
	fmt.Printf("RD: %v\n", RD)

	var RA string
	if flags.RA {
		RA = "Recursion available"
	} else {
		RA = "No recursion available"
	}
	fmt.Printf("RA: %v\n", RA)

	fmt.Printf("Z: %v\n", flags.Z)

	var RCODE string
	switch flags.RCODE {
	case 0:
		RCODE = "No errors"
	case 1:
		RCODE = "Format error"
	case 2:
		RCODE = "Server Error"
	case 3:
		RCODE = "Name Error"
	case 4:
		RCODE = "Not implemented"
	case 5:
		RCODE = "Refused"
	default:
		RCODE = "Future use / todo"
	}
	fmt.Printf("RCODE: %v\n", RCODE)
}

func PrintHeader(header DnsHeader) {
	fmt.Printf("ID: %v\n", header.ID)
	fmt.Println("-- Header flags --")
	PrintHeaderFlags(header.flags)
	fmt.Println("-- End of flags --")
	fmt.Printf("Question count: 		%v\n", header.QDCOUNT)
	fmt.Printf("Answer count: 			%v\n", header.ANCOUNT)
	fmt.Printf("Authoritative count: 	%v\n", header.NSCOUNT)
	fmt.Printf("Additionnal count:		%v\n", header.ARCOUNT)
}

func PrintQuestion(question DnsQuestion) {
	fmt.Printf("Question Name: 		%v\n", question.QNAME)
	fmt.Printf("Question Type: 		%v\n", question.QTYPE)
	fmt.Printf("Question Class: 	%v\n", question.QCLASS)
}

func PrintResourceRecord(rr DnsResourceRecord) {
	fmt.Printf("Name: %v\n", rr.NAME)
	fmt.Printf("Type: %v\n", rr.TYPE)
	fmt.Printf("Class: %v\n", rr.CLASS)
	fmt.Printf("TTL (seconds): %v\n", rr.TTL)
	fmt.Printf("Response data length: %v\n", rr.RDLENGTH)
	PrintRDATA(rr.RDATA)
}

func PrintDnsDatagram(d DnsDatagram) {
	PrintHeader(d.Header)
	var i uint16
	for i = 0; i < d.Header.QDCOUNT; i++ {
		fmt.Printf("-- Question %v --\n", i+1)
		PrintQuestion(d.Questions[i])
		fmt.Printf("-- End of Q %v --\n", i+1)
	}
	for i = 0; i < d.Header.ANCOUNT; i++ {
		fmt.Printf("--- Answer %v ---\n", i+1)
		PrintResourceRecord(d.Answers[i])
		fmt.Printf("-- End of A %v --\n", i+1)

	}
	for i = 0; i < d.Header.NSCOUNT; i++ {
		fmt.Printf("-- Name server %v --\n", i+1)
		PrintResourceRecord(d.Authorities[i])
		fmt.Printf("--- End of NS %v ---\n", i+1)

	}
	for i = 0; i < d.Header.ARCOUNT; i++ {
		fmt.Printf("-- Additional %v --\n", i+1)
		PrintResourceRecord(d.Additionals[i])
		fmt.Printf("-- End of Add %v --\n", i+1)
	}
}

func PrintRDATA(rdata RDATA) {
	var str string
	switch rdata.rdata_type {
	case 1:
		str = FormatA_RDATA(rdata.A)
	default:
		str = fmt.Sprintf("%v", rdata.other.data)
	}
	fmt.Printf("RDATA: %v\n", str)
}

func FormatA_RDATA(A_rdata A_RDATA) string {
	output := fmt.Sprintf("%v.%v.%v.%v", A_rdata.address[0], A_rdata.address[1], A_rdata.address[2], A_rdata.address[3])
	return output
}
