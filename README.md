# pazaak-rs
A rust CLI implementation of pazaak from the KOTOR series

## Getting Started

### Usage
Clone this repository and navigate to the root directory. To play pazaak-rs, you will need to create 2 Side Decks (see [Side Decks](#side-decks)) for each player. Once you have created your Side Decks, you can play pazaak by running the following command:
```
./pazaak-rs <player1_side_deck> <player2_side_deck>
```

### Playing Pazaak
Each turn you will be updated with the state of the board and your hand. You can then choose to _`play`_, _`stand`_, or _`end`_ your turn. 
- Choosing to _`play`_, you will be prompted to select a card from your hand. You can then choose to play the card to your board by entering the card's index. You're welcome to _`cancel`_ your play action at any time.
- _`stand`_, you will end your turn and the board will be updated. 
- _`end`_, you will end the game.

## The Basics
Pazaak will feel very similar to those familiar with blackjack. The goal is to get as close to 20 as possible without going over. What sets Pazaak apart is the ability to manipulate the value of your board. You can play cards from your predetermined hand to increase or decrease the value of your board. The round ends when both players choose to stand. The player with the highest value under or equal to 20 wins.

## Types of Cards
### Standard
Standard cards are the most common type of card. They have a value of 1-10 and can be played to increase the value of your board by that amount. The dealer will distribute a card from the board deck at the start of each round. The board deck contains 4 sets of 1-10.

The player can opt to add standard cards to their Side Deck. Players are also able to include standard cards with negative values, however, the board deck will never contain negative values.

### Specials
Special cards are a bit more complicated. They can be played to increase or decrease the value of your board by a certain amount. These cards originate from the player's hand and can only be played one at a time. Below are the special cards and their effects.

1. Flip (+X/-X)
    - A `Flip` card can have a positive or negative value. When played, the player can choose which value to use. The chosen value will then be added to the board.

2. Double (D)
    - A `Double` card can be played to double the last card played on the board. For example, if the player plays a `Double` card after a card with a value of 2, the `Double` card will become a card with a value of 2 too.

3. Invert (X&X)
    - An `Invert` card can be played to invert the value of existing cards on the board. The card will have two values, each of these values will be inverted. For example, if the board has some cards with values of 1, 2, 3, and 4, and the player plays an `Invert` card with values of 2 and 4, the board will now have cards with values of 1, -2, 3, and -4. Similarly, negative values will be inverted to positive values.

4. TieBreaker (+1/-1T)
    - Extremely similar to a `Flip` card, the `TieBreaker` card can be played to increase or decrease the value of the board. However, if the player's board total is equal to their opponent's board total, the `TieBreaker` card will promote the player to win the round.

## Side Decks
Side Decks are a collection pre-determined cards chosen by the player before gamestart. These cards may be played during a turn to manipulate their board. Side Decks are to be loaded from a .pzk file passed as a CLI argument. When building a deck you will only have access to the following cards:
- 12 blue ‘+’ cards, ranging 1-6
- 12 red ‘-’ cards, ranging 1-6
- 12 blue and red ‘+/-’ cards, ranging 1-6
- 2 yellow ‘2&4’ flip cards
- 2 yellow ‘3&6’ flip cards
- 1 yellow ‘+/-1/2’ card
- 1 yellow ‘double’ card
- 1 yellow ‘tiebreak’ card

Side Decks are represented by a text file with the following format:
```
2
+1/-1T
D
2&4
1
-1
3&6
-3
2
+1/-1
```
Each line hosts a single card. There can only be one card per line and there should be 10 cards in total. Any assortment of cards can be used in a Side Deck but it's recommended to include at least 1 `TieBreaker` card.

## Winning
If a round ends in a tie, no player will receive a point. If a player wins a round, they will receive a point. The first player to reach 3 points will win the game.


# Future Plans
- [ ] Don't refill hand between rounds
- [ ] Add 'fill the table' win condition
- [ ] Restrict deck contents
  - Restrict number of special cards
  - Limit TieBreaker cards to +1/-1T
- [ ] Add opponent AI
- [ ] Improve CLI
  - Add color
  - Add animations
  - Add sounds