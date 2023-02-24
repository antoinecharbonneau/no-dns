package datagram

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUnpackDnsNameFieldSimpleValid(t *testing.T) {
	// Valid test
	input := []byte{3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v', 0}
	expectedOutput := "www.antoinec.dev"
	var expectedOffset uint16 = uint16(len(input))
	name, offset, err := unpackDnsNameField(input, 0)

	assert.Equal(t, expectedOutput, name, "Output should match expected value")
	assert.Equal(t, expectedOffset, offset, "Offset should match expected value")
	assert.Nil(t, err)

	input = []byte{3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v'}
	expectedOutput = ""
	expectedOffset = 0
	name, offset, err = unpackDnsNameField(input, 0)

	assert.Equal(t, expectedOutput, name, "Output should match expected value")
	assert.Equal(t, expectedOffset, offset, "Offset should match expected value")
	assert.NotNil(t, err, "Error should be raised.")

	input = []byte{
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v',
		0,
	}

	expectedOutput = ""
	expectedOffset = 0
	name, offset, err = unpackDnsNameField(input, 0)

	assert.Equal(t, expectedOutput, name, "Output should match expected value")
	assert.Equal(t, expectedOffset, offset, "Offset should match expected value")
	assert.NotNil(t, err, "Error should be raised.")

	input = []byte{3, 'w', 'w', 'w', 8, 'a', 'n', 't', 'o', 'i', 'n', 'e', 'c', 3, 'd', 'e', 'v', 0, 4, 't', 'e', 's', 't', 0xc0, 0x04}
	expectedOutput = "test.antoinec.dev"
	expectedOffset = uint16(len(input))
	name, offset, err = unpackDnsNameField(input, 18)

	assert.Equal(t, expectedOutput, name, "Output should match expected value")
	assert.Equal(t, expectedOffset, offset, "Offset should match expected value")
	assert.Nil(t, err)
}
