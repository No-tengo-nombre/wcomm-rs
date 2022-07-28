use std::fmt;

pub struct Message {
    _data: String,
    _header: String,
    _total: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.as_string());
    }
}

impl Message {
    pub fn new() -> Message {
        return Message {
            _data: "".to_string(),
            _header: "".to_string(),
            _total: "".to_string(),
        };
    }

    pub fn from_string(data: &str, header: &str) -> Message {
        return Message {
            _data: string_to_binary(&data),
            _header: string_to_binary(&header),
            _total: string_to_binary(&data) + &string_to_binary(&header),
        };
    }

    pub fn from_binary(data: &str, header: &str) -> Message {
        return Message {
            _data: data.to_string(),
            _header: header.to_string(),
            _total: data.to_string() + &header.to_string(),
        };
    }

    pub fn data(mut self, data: &str) -> Message {
        self._data = string_to_binary(&data);
        self._total = self._header.clone() + &self._data;
        return self;
    }

    pub fn data_from_binary(mut self, data: &str) -> Message {
        self._data = data.to_string();
        self._total = self._header.clone() + &self._data;
        return self;
    }

    pub fn header(mut self, header: &str) -> Message {
        self._header = string_to_binary(&header.to_string());
        self._total = self._header.clone() + &self._data;
        return self;
    }

    pub fn header_from_binary(mut self, header: &str) -> Message {
        self._header = header.to_string();
        self._total = self._header.clone() + &self._data;
        return self;
    }

    pub fn get_data(&self) -> &str {
        return &self._data;
    }

    pub fn get_header(&self) -> &str {
        return &self._header;
    }

    pub fn bit_size(&self) -> u32 {
        return (self._total.len()) as u32;
    }

    pub fn data_size(&self) -> u32 {
        return self._data.len() as u32;
    }

    pub fn header_size(&self) -> u32 {
        return self._header.len() as u32;
    }

    pub fn group_string(message: &String, num: u32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut temp = message.clone();
        while temp != "" {
            result.push(temp[..(num as usize)].to_string());
            temp = temp[(num as usize)..].to_string();
        }
        return result;
    }

    pub fn group(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._total, num);
    }

    pub fn group_data(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._data, num);
    }

    pub fn group_header(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._header, num);
    }

    pub fn as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    pub fn data_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_data(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    pub fn header_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_header(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    pub fn as_binary(&self) -> &str {
        return &self._total;
    }

    pub fn data_as_binary(&self) -> &str {
        return &self._data;
    }

    pub fn header_as_binary(&self) -> &str {
        return &self._header;
    }

    pub fn as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    pub fn data_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.data_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    pub fn header_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.header_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }
}

pub fn string_to_binary(data: &str) -> String {
    let mut result = "".to_string();
    for c in data.bytes() {
        result += &format!("{c:0>8b}");
    }
    return result;
}
