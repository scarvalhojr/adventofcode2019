use intcode2::{InputOutput, IntcodeComputer};
use std::collections::VecDeque;
use std::convert::TryFrom;

const NAT_ADDR: i64 = 255;

type Address = i64;
type Packet = (i64, i64);
type Message = (Address, Packet);
type MessageQueue = VecDeque<Message>;

struct Computer {
    nic: IntcodeComputer,
    nic_io: NicInputOutput,
}

struct NicInputOutput {
    incoming: VecDeque<i64>,
    outgoing: Vec<i64>,
}

impl Computer {
    fn new(address: Address, nic_program: &[i64]) -> Self {
        let nic = IntcodeComputer::new(nic_program, true);
        let nic_io = NicInputOutput::new(address);
        Self { nic, nic_io }
    }

    fn receive(&mut self, packet: Packet) {
        let (value_x, value_y) = packet;
        self.nic_io.incoming.push_back(value_x);
        self.nic_io.incoming.push_back(value_y);
    }

    fn send(&mut self, message_queue: &mut MessageQueue) -> Option<()> {
        self.nic_io.incoming.push_back(-1);
        self.nic.run(&mut self.nic_io)?;
        for messsage in self.nic_io.outgoing.as_slice().chunks(3) {
            let dest_address = messsage[0];
            let packet = (messsage[1], messsage[2]);
            message_queue.push_back((dest_address, packet));
        }
        self.nic_io.outgoing.clear();
        Some(())
    }
}

impl NicInputOutput {
    fn new(address: Address) -> Self {
        let incoming = [address].iter().copied().collect();
        let outgoing = Vec::new();
        Self { incoming, outgoing }
    }
}

impl InputOutput for NicInputOutput {
    fn provide_input(&mut self) -> Option<i64> {
        self.incoming.pop_front()
    }
    fn take_output(&mut self, value: i64) -> Option<()> {
        self.outgoing.push(value);
        Some(())
    }
}

pub fn run_network(nic_program: &[i64]) -> Option<(i64, i64)> {
    let mut message_queue = MessageQueue::new();
    let mut computers = Vec::new();

    for address in 0..50 {
        // Start each computer with a unique address
        let mut computer = Computer::new(address, nic_program);
        computer.send(&mut message_queue)?;
        computers.push(computer);
    }

    let mut first_nat_packet = None;
    let mut last_nat_packet = None;
    let mut last_resume_packet = None;

    loop {
        while let Some((dest_address, packet)) = message_queue.pop_front() {
            if dest_address == NAT_ADDR {
                if first_nat_packet.is_none() {
                    // First packet to NAT
                    first_nat_packet = Some(packet);
                }
                last_nat_packet = Some(packet);
            } else {
                let computer_address = usize::try_from(dest_address).ok()?;
                if let Some(computer) = computers.get_mut(computer_address) {
                    computer.receive(packet);
                    computer.send(&mut message_queue)?;
                }
            }
        }
        // All computers are now idle: first check
        // we have a NAT packet to resume activity
        last_nat_packet?;
        if last_resume_packet == last_nat_packet {
            // Found first repeated resume package
            let (_, part1) = first_nat_packet.unwrap();
            let (_, part2) = last_resume_packet.unwrap();
            return Some((part1, part2));
        }
        // Send last NAT packet to address 0 to resume activity
        message_queue.push_back((0, last_nat_packet.unwrap()));
        last_resume_packet = last_nat_packet;
    }
}
