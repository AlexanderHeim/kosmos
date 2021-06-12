use crate::{Serialization, connection::Connection, neuron::Neuron};

#[derive(Debug)]
pub struct Network {
    amount_in: u32,
    amount_out: u32,
    neurons: Vec<Neuron>,
    layers: Vec<Vec<Connection>>,
}

impl Serialization<Network> for Network {
    fn deserialize(s: &str) -> Network {
        let split: Vec<&str> = s.split(";").collect();
        let mut connections: Vec<Connection> = Vec::new();
        let mut neurons: Vec<Neuron> = Vec::new();
        for s in split {
            if s.starts_with("C") {
                connections.push(Connection::deserialize(s));
            } else if s.starts_with("N") {
                neurons.push(Neuron::deserialize(s));
            } 
        }
        neurons.sort_by(|a, b| a.id.cmp(&b.id));
        connections.sort_by(|a, b| a.id.cmp(&b.id));
        let amount_in = neurons.iter().filter(|n| n.id == 0).count() as u32;
        let amount_out = neurons.iter().filter(|n| n.id == 1).count() as u32;

        //let in_ids: Vec<u32> = neurons.iter().filter(|n| n.id == 0).map(|n| n.id).collect();
        let mut traced_ids: Vec<u32> = neurons.iter().filter(|n| n.ntype == 1).map(|n| n.id).collect();
        let mut layers: Vec<Vec<Connection>> = Vec::new();
        layers.push(Vec::new());
        let mut depth = 0;
        while !connections.is_empty() {
            let mut current_ids: Vec<u32> = Vec::new();
            let len = connections.len();
            let mut counter: i32 = 0;
            while counter < len as i32 {
                
                if counter >= connections.len() as i32 {
                    break;
                }
                if traced_ids.contains(&connections[counter as usize].on_id) {
                    if !current_ids.contains(&connections[counter as usize].in_id) {
                        current_ids.push(connections[counter as usize].in_id);
                        
                    }
                    layers[depth].push(connections.remove(counter as usize));
                    counter -= 1;
                }
                counter += 1;
            }
            depth += 1;
            layers.push(Vec::new());
            traced_ids.append(&mut current_ids);
        }
        Network {
            amount_in,
            amount_out,
            neurons,
            layers,
        }
    }

    fn serialize(&self) -> String {
        let mut result = String::new();
        result.extend(self.neurons.iter().map(|n| format!("{};", n.serialize())));
        for l in &self.layers {
            result.extend(l.iter().map(|c| format!("{};", c.serialize())));
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::{Network, Serialization};

    #[test]
    pub fn serialization_test_1() {
        let n = "N0,0,0,0;N1,1,0,0;C0,0,0,1,1.5,2;";
        let network = Network::deserialize(n);
        let s = network.serialize();
        assert_eq!(n, s);
    }

    #[test]
    pub fn serialization_test_2() {
        let n = "N0,0,0,0;N1,0,0,0;N2,2,0,0;N3,2,0,0;N4,1,0,0;C5,0,2,4,1,0;C6,0,3,4,1,0;C1,0,0,2,1,0;C2,0,1,3,1,0;C3,0,0,3,1,0;C4,0,1,2,1,0;";
        let network = Network::deserialize(n);
        //println!("{:#?}", network);
        let s = network.serialize();
        assert_eq!(n, s);
    }
}