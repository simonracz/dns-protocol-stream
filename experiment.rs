use std::net::UdpSocket;

/*
                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+


[0, 1 == ID
129 == 10000001
128 == 10000000 == no error
0, 1 == QSection
0, 1 == ASection
0, 0
0, 0
6, 103, 111, 111, 103, 108, 101 == google
3, 99, 111, 109, 0, == com.
0, 1 == A
0, 1 == IN
192 == 11000000, 12 == pointer to 12th byte of message => google.com
0, 1 == A
0, 1 == IN
0, 0, 0, 186 == TTL in seconds
0, 4 = 4 byte length
142, 251, 39, 110 == IP ADDRESS
]

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                                               /
    /                      NAME                     /
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     CLASS                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TTL                      |
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                   RDLENGTH                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
    /                     RDATA                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+


*/

fn main() {
    let id = 1u16;
    let qflags = 0b0_0000_0010_000_0000u16;
    let counts = [0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
    let qtype = 1u16; // A record, ipv4
    let qclass = 1u16; // IN - Internet
    let qname = [6u8, b'g', b'o', b'o', b'g', b'l', b'e', 3u8, b'c', b'o', b'm', 0u8];

    let mut qmsg = vec![];
    qmsg.extend_from_slice(&id.to_be_bytes());
    qmsg.extend_from_slice(&qflags.to_be_bytes());
    qmsg.extend_from_slice(&counts);
    qmsg.extend_from_slice(&qname);
    qmsg.extend_from_slice(&qtype.to_be_bytes());
    qmsg.extend_from_slice(&qclass.to_be_bytes());

    let socket = UdpSocket::bind(("0.0.0.0", 0)).expect("Failed to bind UDP socket");
    socket.connect(("1.1.1.1", 53)).expect("Failed to connect to dns server.");
    socket.send(&qmsg).expect("Failed to send query to dns server.");
    let mut buf = [0; 512];
    match socket.recv(&mut buf) {
        Ok(received) => {
            println!("Received {} bytes {:?}\n", received, &buf[..received]);
        },
        Err(err) => {
            println!("Recv failed: {:?}\n", err);
            return;
        }
    };
}
