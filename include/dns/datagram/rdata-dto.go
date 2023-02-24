package datagram

type A_RDATA struct {
	address [4]uint8
}

// type NS_RDATA struct {
// 	NAME string
// }

// type MX_RDATA struct {
// 	PREFERENCE uint16
// 	NAME       string
// }

type OTHER_RDATA struct {
	data []byte
}

type RDATA struct {
	rdata_type uint16
	A          A_RDATA
	other      OTHER_RDATA
}
