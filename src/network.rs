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
        let amount_in = neurons.iter().filter(|n| n.ntype == 0).count() as u32;
        let amount_out = neurons.iter().filter(|n| n.ntype == 1).count() as u32;

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

impl Network {
    pub fn feed(&mut self, food: &[f32]) -> Vec<f32> {
        for i in 0..self.amount_in {
            self.neurons[i as usize].value = food[i as usize];
        }
        let amount_layers = self.layers.len();
        for l in 1..(amount_layers+1) {
            for c in &self.layers[amount_layers-l] {
                let id_in = c.in_id;
                let id_on = c.on_id;
                self.neurons[id_on as usize].value += c.feed(self.neurons[id_in as usize].value);
            }
        }
        let mut result: Vec<f32> = Vec::new();
        for i in 1..(self.amount_out+1) {
            result.push(self.neurons[self.neurons.len()-i as usize].value);
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
        let n = "N0,0,0,0;N1,0,0,0;N2,2,0,0;N3,2,0,0;N4,1,0,0;C4,0,2,4,2,0;C6,0,3,4,1,0;C0,0,0,2,1,0;C1,0,1,3,1,0;C2,0,0,3,1,0;C3,0,1,2,1,0;";
        let network = Network::deserialize(n);
        let s = network.serialize();
        assert_eq!(n, s);
    }

    #[test]
    pub fn feed_test_1() {
        let n = "N0,0,0,0;N1,1,0,0;C0,0,0,1,3,2;";
        let mut network = Network::deserialize(n);
        let r = network.feed(&[3.0]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], 11.0);
    }

    #[test]
    pub fn feed_test_2() {
        let n = "N0,0,0,0;N1,2,0,0;N2,2,0,0;N3,1,0,0;C0,0,0,1,3,2;C1,0,0,2,-3,4;C2,0,1,3,2.57,2.332;C3,0,2,3,-1.043,-0.00012;";
        let mut network = Network::deserialize(n);
        let r = network.feed(&[1.42]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], 18.69126);
    }

    #[test]
    pub fn feed_test_3() {
        let n = "N0,0,0,0;N1,0,0,0;N2,2,0,0;N3,1,0,0;N4,1,0,0;C0,0,0,3,2,1;C1,0,0,2,1.5,-2;C2,0,1,2,-0.4,0;C3,0,2,3,-0.004,1;C4,0,2,4,0.5,-0.2;";
        let mut network = Network::deserialize(n);
        let r = network.feed(&[1.42, 2.34]);
        assert_eq!(r.len(), 2);
        assert_eq!(format!("{:.3}", r[0]), format!("{}", -0.603));
        assert_eq!(format!("{:.6}", r[1]), format!("{}", 4.843224));
    }
}