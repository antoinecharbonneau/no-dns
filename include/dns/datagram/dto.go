package datagram

// Detailed documentation : https://www.ietf.org/rfc/rfc1035.txt

type DnsDatagram struct {
	Header      DnsHeader
	Questions   []DnsQuestion
	Answers     []DnsResourceRecord
	Authorities []DnsResourceRecord
	Additionals []DnsResourceRecord
}

type DnsHeader struct {
	ID      uint16
	flags   DnsHeaderFlags
	QDCOUNT uint16
	ANCOUNT uint16
	NSCOUNT uint16
	ARCOUNT uint16
}

type DnsHeaderFlags struct {
	// Query / Response
	// False: Query
	// True: Response
	QR bool

	// Operation Code [0-15]
	// 0: QUERY
	// 1: IQUERY
	// 2: STATUS
	// 3-15: Future use
	OPCODE uint8

	// Authoritative Answer
	// False: not authoritative
	// True: authoritative answer
	AA bool

	// Truncation
	// False: not truncated
	// True: Truncated due to higher length than channel allows
	TC bool

	// Recursion desired
	// False: No recursion desired
	// True: Recursion desired - Name server should pursue query recursively
	RD bool

	// Recursion available
	// False: Recursion is not available
	// True: Recursion is available
	RA bool

	// Future use, must be 0 always
	Z uint8

	// Response code
	// 0: No error condition
	// 1: Format error - Query was not able to be interpreted
	// 2: Server failure - Server was not able to be processed
	// 3: Name error - From authoritative answer, does not exist
	// 4: Not implemented - ¯\_(ツ)_/¯
	// 5: Refused
	// 6-15: Future use
	RCODE uint8
}

type DnsQuestion struct {
	QNAME  string
	QTYPE  uint16
	QCLASS uint16
}

type DnsResourceRecord struct {
	NAME string

	// TYPE            value and meaning
	// A               1 a host address
	// NS              2 an authoritative name server
	// MD              3 a mail destination (Obsolete - use MX)
	// MF              4 a mail forwarder (Obsolete - use MX)
	// CNAME           5 the canonical name for an alias
	// SOA             6 marks the start of a zone of authority
	// MB              7 a mailbox domain name (EXPERIMENTAL)
	// MG              8 a mail group member (EXPERIMENTAL)
	// MR              9 a mail rename domain name (EXPERIMENTAL)
	// NULL            10 a null RR (EXPERIMENTAL)
	// WKS             11 a well known service description
	// PTR             12 a domain name pointer
	// HINFO           13 host information
	// MINFO           14 mailbox or mail list information
	// MX              15 mail exchang
	// TXT             16 text strings
	TYPE uint16

	// Class of record
	// 1: IN / Internet
	// 2: CS / CSNET (Obsolete)
	// 3: CH / Chaos
	// 4: HS / Hesiod (whatever that is)
	CLASS    uint16
	TTL      int32
	RDLENGTH uint16
	RDATA    RDATA
}
