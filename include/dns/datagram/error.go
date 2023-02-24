package datagram

type NameTooLong struct {
	message string
}

func (z NameTooLong) Error() string {
	return "Name field is over 255 bytes"
}

type NameNoEnd struct {
	message string
}

func (z NameNoEnd) Error() string {
	return "Did not find NULL byte to terminate name before end of stream"
}
