package proxy

import (
	"datagram"
	"fmt"
	"log"
	"net"
)

func RunDnsProxy() {
	udpServer, err := net.ListenPacket("udp", ":1053")
	if err != nil {
		log.Fatal(err)
	}
	defer udpServer.Close()

	for {
		buf := make([]byte, 1024)
		_, addr, err := udpServer.ReadFrom(buf)
		if err != nil {
			continue
		}
		go response(udpServer, addr, buf)
	}
}

func response(udpServer net.PacketConn, addr net.Addr, buf []byte) {
	requestLog := fmt.Sprintf("Request from %v ", addr.String())
	dnsData, err := datagram.UnpackDnsDatagram(buf)
	if err != nil {
		requestLog += fmt.Sprintf("resulted in error : %v\n", err.Error())
	} else {
		requestLog += fmt.Sprintf("for %v\n", dnsData.Questions[0].QNAME)
	}
	log.Print(requestLog)
	datagram.PrintDnsDatagram(dnsData)
	response := []byte(requestLog)
	udpServer.WriteTo(response, addr)
}
