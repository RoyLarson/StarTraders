
use std::io;

pub fn query_to_display_instructions(){
    println!("DOES ANYONE NEED INSTRUCTIONS: Y/y (yes)");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Did not get an answer");
    input = input.trim().to_string();

    match input.as_str()=="y" {
        true=>{
            print_instructions();
        }
        false=>{}
    }
}

pub fn print_instructions() {
    let page1 = concat!(
        "   STAR LANES IS A GAME OF INTERSTELLAR TRADING.\n",
        "THE OBJECT OF THE GAME IS TO AMASS THE GREATEST AMOUNT\n",
        "OF MONEY.  THIS IS ACCOMPLISHED BY ESTABLISHING VAST,\n",
        "INTERSTELLAR SHIPPING LANES, AND PURCHASING STOCK IN\n",
        "THE COMPANIES THAT CONTROL THOSE TRADE ROUTES.  DURING\n",
        "THE COURSE OF THE GAME, STOCK APPRECIATES IN VALUE AS\n",
        "THE SHIPPING COMPANIES BECOME LARGER.  ALSO, SMALLER\n",
        "COMPANIES CAN BE MERGED INTO LARGER ONES, AND STOCK\n",
        "IN THE SMALLER FIRM IS CONVERTED INTO STOCK IN THE\n",
        "LARGER ONE AS DESCRIBED BELOW.\n",
        "EACH TURN, THE COMPUTER WILL PRESENT THE PLAYER WITH\n",
        "FIVE PROSPECTIVE SPACES TO OCCUPY ON A 9X12 MATRIX\n",
        "(ROWS 1-9, COLUMNS A-L).  THE PLAYER, AFTER EXAMINING\n",
        "THE MAP OF THE GALAXY TO DECIDE WHICH SPACE HE WISHES\n",
        "TO OCCUPY, RESPONDS WITH THE ROW AND COLUMN OF THAT\n",
        "SPACE, I.E., 7E, 8A, ETC.  THERE ARE FOUR POSSIBLE\n",
        "MOVES A PLAYER CAN MAKE.\n\n"
    );

    let page2 = concat!(
        "   1. HE CAN ESTABLISH AN UNATTACHED OUTPOST- IF HE\n",
        "SELECTS A SPACE THAT IS NOT ADJACENT TO A STAR, ANOTHER\n",
        "UNATTACHED OUTPOST, OR AN EXISTING SHIPPING LANE, THIS\n",
        "SPACE WILL BE DESIGNATED WITH A '+'.  HE WILL THEN PROCEED\n",
        "WITH STOCK TRANSACTIONS, AS LISTED BELOW.\n",
        "   2. HE CAN ADD TO AN EXISTING LANE- IF HE SELECTS A\n",
        "SPACE THAT IS ADJACENT TO ONE - AND ONLY ONE EXISTING\n",
        "SHIPPING LANE, THE SPACE HE SELECTS WILL BE ADDED TO\n",
        "THAT SHIPPING LANE AND WILL BE DESIGNATED WITH THE FIRST\n",
        "LETTER OF THE COMPANY THAT OWNS THAT LANE.  IF THERE ARE\n",
        "ANY STARS OR UNATTACHED OUTPOSTS ALSO ADJACENT TO THE\n",
        "SELECTED SPACE, THEY, TOO, WILL BE INCORPORATED INTO THE\n",
        "EXISTING LANE.  EACH NEW SQUARE ADJACENT TO A STAR ADDS\n",
        "$500 PER SHARE, AND EACH NEW OUTPOST ADDS $100 PER SHARE\n",
        "TO THE MARKET VALUE OF THE STOCK OF THAT COMPANY.\n"
    );

    let page3 = concat!(
        "   3. HE MAY ESTABLISH A NEW SHIPPING LANE- IF THERE\n",
        "ARE FIVE OR LESS EXISTING SHIPPING LANES ESTABLISHED,\n",
        "THE PLAYER MAY, GIVEN THE PROPER SPACE TO PLAY, ESTABLISH\n",
        "A NEW SHIPPING LANE.  HE MAY DO THIS BY OCCUPYING A SPACE\n",
        "ADJACENT TO A STAR OR ANOTHER UNATTACHED OUTPOST, BUT\n",
        "NOT ADJACENT TO AN EXISTING SHIPPING LANE.  IF HE\n",
        "ESTABLISHES A NEW SHIPPING LANE, HE IS AUTOMATICALLY\n",
        "ISSUED 5 SHARES IN THE NEW COMPANY AS A REWARD.  HE\n",
        "MAY THEN PROCEED TO BUY STOCK IN ANY ACTIVE COMPANY,\n",
        "INCLUDING THE ONE JUST FORMED, AS DESCRIBED BELOW.\n",
        "THE MARKET VALUE OF THE NEW STOCK IS ESTABLISHED BY\n",
        "THE NUMBER OF STARS AND OCCUPIED SPACES AS DESCRIBED\n",
        "IN #2 ABOVE.\n"
    );
    let page4 = concat!(
        "   4. HE MAY MERGE TWO EXISTING COMPANIES- IF A PLAYER\n",
        "SELECTS A SPACE ADJACENT TO TWO EXISTING SHIPPING\n",
        "LANES, A MERGER OCCURS.  THE LARGER COMPANY TAKES OVER THE\n",
        "SMALLER COMPANY - (IF BOTH COMPANIES ARE THE SAME SIZE\n",
        "PRIOR TO THE MERGER, THEN THE SURVIVOR IS DETERMINED BY\n",
        "ALPHABETICAL ORDER OF THE TWO COMPANY NAMES - THE EARLIER\n",
        "SURVIVES).  THE STOCK OF THE SURVIVING COMPANY IS\n",
        "INCREASED IN VALUE ACCORDING TO THE NUMBER OF SPACES\n",
        "AND STARS ADDED TO ITS LANE.  EACH PLAYERS STOCK IN\n",
        "THE DEFUNCT COMPANY IS EXCHANGED FOR SHARES IN THE\n",
        "SURVIVOR ON A RATIO OF 2 FOR 1.  ALSO, EACH PLAYER\n",
        "IS PAID A CASH BONUS PROPORTIONAL TO THE PERCENTAGE\n",
        "OF OUTSTANDING STOCK HE HELD IN THE DEFUNCT COMPANY.\n",
        "NOTE: AFTER A COMPANY BECOMES DEFUNCT THROUGH THE\n",
        "MERGER PROCESS, IT CAN REAPPEAR ELSE WHERE ON THE\n",
        "BOARD WHEN, AND IF, A NEW COMPANY IS ESTABLISHED.\n"
    );

    let page5= concat!(
        "   NEXT THE COMPUTER ADDS STOCK DIVIDENDS TO THE PLAYER'S\n",
        "CASH ON HAND (5% OF THE MARKET VALUE OF THE STOCK IN HIS\n",
        "POSSESSION), AND OFFERS HIM THE OPPORTUNITY TO PURCHASE\n",
        "STOCK IN ANY OF THE ACTIVE COMPANIES ON THE BOARD.\n",
        "STOCK MAY NOT BE SOLD, BUT THE MARKET VALUE OF EACH\n",
        "PLAYER'S STOCK IS TAKEN INTO ACCOUNT AT THE END OF THE\n",
        "GAME TO DETERMINE THE WINNER.  IF THE MARKET VALUE OF A GIVEN\n",
        "STOCK EXCEEDS $3000 AT ANY TIME DURING THE GAME, THAT\n",
        "STOCK SPLITS 2 FOR 1.  THE PRICE IS CUT IN HALF, AND \n",
        "THE NUMBER OF SHARES OWNED BY EACH PLAYER IS DOUBLED.\n\n",
        "NOTE:  THE PLAYER MAY LOOK AT HIS PORTFOLIO AT ANY TIME\n",
        "DURING THE COURSE OF HIS TURN BY RESPONDING WITH 'STOCK'\n",
        "TO AN INPUT STATEMENT.  LIKEWISE, HE CAN REVIEW THE MAP\n",
        "OF THE GALAXY BY TYPING 'MAP' TO AN INPUT STATEMENT.\n"
    );

    let page6= concat!(
        "\t\t** GAME ENDS AFTER 48 MOVES **\n\n\n",
        "PLAYER WITH THE GREATEST NET WORTH AT THAT POINT IS THE WINNER."
    );

    let instructions: Vec<&str> = vec![page1, page2, page3, page4, page5, page6 ];
    let mut cont = String::new();
    for inst in instructions{
        println!("{}", inst);
        println!("Press enter to continue:");
        match io::stdin().read_line(&mut cont){
            Ok(_n)=>{continue}
            Err(_err)=>{continue}
        }
    }

}

