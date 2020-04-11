pub struct Genie {
        pub runs :u16,
        pub wickets :u16,
        pub bowled_balls :u16,
        pub required_runrate :f32,
        pub current_runrate :f32,
        pub status : super::utils::PlayerStatus,
        pub won_game : super::utils::GameStatus,
}