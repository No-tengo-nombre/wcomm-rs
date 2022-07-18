pub struct Message {
    _data: String,
    _header: String,
}

impl Message {
    fn new() -> Message {
        return Message {
            _data: "".to_string(),
            _header: "".to_string(),
        };
    }

    fn from_string(data: &String, header: &String) -> Message {
        return Message {
            _data: string_to_binary(data),
            _header: string_to_binary(header),
        };
    }

    fn from_binary(data: &String, header: &String) -> Message {
        return Message {
            _data: data.to_string(),
            _header: header.to_string(),
        };
    }

    fn data(mut self, data: &String) -> Message {
        self._data = string_to_binary(data);
        return self;
    }

    fn data_from_binary(mut self, data: &String) -> Message {
        self._data = data.to_string();
        return self;
    }

    fn header(mut self, header: &String) -> Message {
        self._header = string_to_binary(header);
        return self;
    }

    fn header_from_binary(mut self, header: &String) -> Message {
        self._header = header.to_string();
        return self;
    }

    fn get_data(&self) -> String {
        return self._data.clone();
    }

    fn get_header(&self) -> String {
        return self._header.clone();
    }

    fn bit_size(&self) -> u32 {
        return (self._header.len() + self._data.len()) as u32;
    }

    fn data_size(&self) -> u32 {
        return self._data.len() as u32;
    }

    fn header_size(&self) -> u32 {
        return self._header.len() as u32;
    }

    fn group_string(message: &String, num: u32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut temp = message.clone();
        while temp != "" {
            result.push(temp[..(num as usize)].to_string());
            temp = temp[(num as usize)..].to_string();
        }
        return result;
    }

    fn group(&self, num: u32) -> Vec<String> {
        return Message::group_string(&(self._header.clone() + &self._data), num);
    }

    fn group_data(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._data, num);
    }

    fn group_header(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._header, num);
    }

    fn as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group(8) {
            result.push(isize::from_str_radix(&c, 2).unwrap().try_into().unwrap());
        }
        return result;
    }

    fn data_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_data(8) {
            result.push(isize::from_str_radix(&c, 2).unwrap().try_into().unwrap());
        }
        return result;
    }

    fn header_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_header(8) {
            result.push(isize::from_str_radix(&c, 2).unwrap().try_into().unwrap());
        }
        return result;
    }

    fn as_binary(&self) -> String {
        return self._header.clone() + &self._data;
    }

    fn data_as_binary(&self) -> String {
        return self._data.clone();
    }

    fn header_as_binary(&self) -> String {
        return self._header.clone();
    }

    fn as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    fn data_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.data_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    fn header_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.header_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }
}

pub fn string_to_binary(data: &String) -> String {
    let mut result = "".to_string();
    for c in data.bytes() {
        result += &format!("0{c:b}");
    }
    return result;
}
