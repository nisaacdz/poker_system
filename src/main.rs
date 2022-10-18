use pokers::member::{Dealer, Player, Center};

#[warn(unused_variables, dead_code)]
fn main(){
    let mut dealer = Dealer::new();

    let mut players = Player::create(4, 100000);
    
    let mut center: Center = Center::new();

    dealer.shuffle_cards();

    deal_to_players(&mut players, &dealer);
    deal_to_center(&mut center, &dealer);
    
    for x in 0..players.len(){
        println!("Player {}: {:?}", x + 1, players.get(x).unwrap().cards);
    }

    println!();

    println!("Center : {:?}", center.cards);

}

fn deal_to_players<'x>(players: &mut Vec<Player<'x>>, dealer: &'x Dealer){
    for _ in 0..2{
        for each_player in players.iter_mut(){
            each_player.cards.push(dealer.draw_next());
        }
    }
}

fn deal_to_center<'y>(center: &mut Center<'y>, dealer: &'y Dealer){
    for _ in 0..3{
        center.cards.push(dealer.draw_next());
    }
}