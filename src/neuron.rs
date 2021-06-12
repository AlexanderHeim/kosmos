use crate::Serialization;

#[derive(Debug)]
pub struct Neuron {
    pub id: u32,
    pub ntype: u8,
    pub bias: f32,
    pub af: u8,
    pub value: f32,
}

impl Serialization<Neuron> for Neuron {
    fn deserialize(s: &str) -> Neuron {
        let s = match s.strip_prefix("N") {
            Some(s) => s,
            None => s,
        };
        let splits: Vec<&str> = s.split(",").collect();
        Neuron {
            id: splits[0].parse::<u32>().unwrap(),
            ntype: splits[1].parse::<u8>().unwrap(),
            bias: splits[2].parse::<f32>().unwrap(),
            af: splits[3].parse::<u8>().unwrap(),
            value: 0.0
        }
    }

    fn serialize(&self) -> String {
        format!("N{},{},{},{}", self.id, self.ntype, self.bias, self.af)
    }
}


#[cfg(test)]
mod test {
    use super::Neuron;
    use super::Serialization;

    #[test]
    pub fn serialization_test_1() {
        let n = "3,0,2.3,1";
        let neuron = Neuron::deserialize(n);
        let s = neuron.serialize();
        assert_eq!(&format!("N{}", n), &s);
        assert_eq!(neuron.id, 3);
        assert_eq!(neuron.ntype, 0);
        assert_eq!(neuron.bias, 2.3);
        assert_eq!(neuron.af, 1);
    }

    #[test]
    pub fn serialization_test_2() {
        let n = "N39239,4,-0.3241323,20";
        let neuron = Neuron::deserialize(n);
        let s = neuron.serialize();
        assert_eq!(n, &s);
        assert_eq!(neuron.id, 39239);
        assert_eq!(neuron.ntype, 4);
        assert_eq!(neuron.bias, -0.3241323);
        assert_eq!(neuron.af, 20);
    }
}