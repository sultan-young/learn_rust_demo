pub struct Hero {
    pub name: String,
    pub hp: u32,
    pub mp: u32,
    pub speed: u32,
    pub skill_ids: Vec<u32>,

}
impl Hero {
    pub fn attack(&self) {
        println!("{} 发起了一次平平无奇的攻击", self.name);
    }
}