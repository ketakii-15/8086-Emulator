/// A Helper to map the character Index to newlines
/// Used as helper, because LALRPOP does not directly give line number, but character number
/// So this is used to initially go over complete input and store occurrences of newlines
/// which can later be used to check the newline that occurred before given character
#[derive(Default)]
pub struct LexerHelper {
    pub temp_line: u16,
    newline_list: Vec<u16>,
}

impl LexerHelper {
    /// Create new Lexer Helper, by scanning input and storing occurrences of newlines
    /// Arguments
    /// input : &str which is input
    pub fn new(input: &str) -> Self {
        let mut l = LexerHelper::default();
        for (i, c) in input.chars().enumerate() {
            if c == '\n' {
                l.newline_list.push(i as u16);
            }
        }
        l
    }

    /// Function to get the number of newline char, which will correspond to line number,
    /// and the place of the newline char.
    /// Arguments :
    /// i : u16 place before which the newline is to be checked
    /// Returns :
    /// (line number, newline char index) : (u16,u16)
    pub fn get_newline_before(&self, i: u16) -> (u16, u16) {
        for (idx, v) in self.newline_list.iter().enumerate() {
            if *v > i {
                return (idx as u16, *v);
            }
        }
        let max = self.newline_list.len();
        return (max as u16, self.newline_list[max - 1]);
    }
}
