use crate::Serialization;

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub ctype: u8,
    pub in_id: u32,
    pub on_id: u32,
    pub weight: f32,
    pub bias: f32,
}

impl Serialization<Connection> for Connection {
    fn deserialize(s: &str) -> Connection {
        let s = match s.strip_prefix("C") {
            Some(s) => s,
            None => s,
        };
        let splits: Vec<&str> = s.split(",").collect();
        Connection {
            id: splits[0].parse::<u32>().unwrap(),
            ctype: splits[1].parse::<u8>().unwrap(),
            in_id: splits[2].parse::<u32>().unwrap(),
            on_id: splits[3].parse::<u32>().unwrap(),
            weight: splits[4].parse::<f32>().unwrap(),
            bias: splits[5].parse::<f32>().unwrap(),
        }
    }

    fn serialize(&self) -> String {
        format!("C{},{},{},{},{},{}", self.id, self.ctype, self.in_id, self.on_id, self.weight, self.bias)
    }
}

impl Connection {
    pub fn feed(&self, input: f32) -> f32 {
        self.weight * input + self.bias
    }
}

#[cfg(test)]
mod test {
    use crate::connection::Connection;
    use super::Serialization;

    #[test]
    pub fn serialization_test_1() {
        let c = "3,0,1,3,1,2";
        let connection = Connection::deserialize(c);
        let s = connection.serialize();
        assert_eq!(&format!("C{}", c), &s);
        assert_eq!(connection.id, 3);
        assert_eq!(connection.ctype, 0);
        assert_eq!(connection.in_id, 1);
        assert_eq!(connection.on_id, 3);
        assert_eq!(connection.weight, 1.0);
        assert_eq!(connection.bias, 2.0);
    }

    #[test]
    pub fn serialization_test_2() {
        let c = "C33992,3,4293,8999,-3.2142,-1.3291";
        let connection = Connection::deserialize(c);
        let s = connection.serialize();
        assert_eq!(c, &s);
        assert_eq!(connection.id, 33992);
        assert_eq!(connection.ctype, 3);
        assert_eq!(connection.in_id, 4293);
        assert_eq!(connection.on_id, 8999);
        assert_eq!(connection.weight, -3.2142);
        assert_eq!(connection.bias, -1.3291);
    }
}