5 REM                   STAR TRADERS
10 REM - MODIFIED FOR 'ALTAIR BASIC 4.0' BY - S J SINGER
20 REM - MODIFIED FOR THE MICROBEE BY JOHN ZAITSEFF, 1988
30 REM
35 DEFINT C
36 DEFINT I
40 DIM M(10,13),S(5,4),N$(5),D1(5),S1(5),Q(5),M$(12),C$(25)
' M = MAP
' S = Company to player holdings
' N$ player names
' D1 = TODO: figure out what
' S1 = Company stock values
' Q = TODO: figure out what
' M$ = Map column names
' C$ = TODO: figure out what
50 DIM C1$(25),C2$(25),B(5) 'B what not originally dimmed
' C1$ = TODO: figure out what
' C2$ = TODO: figure out what
' B = TODO: figure out what
60 DATA 1,"'ALTAIR STARWAYS'"
70 DATA 2,"'BETELGEUSE,LTD.'"
80 DATA 3,"'CAPELLA FREIGHT CO.'"
90 DATA 4,"'DENEBOLA SHIPPERS'"
100 DATA 5,"'ERIDANI EXPEDITERS'"
110 Z1$=CHR$(26)+CHR$(0)
120 PRINT Z1$
130   FOR I=1 TO 5
140    FOR J=1 TO 4
150 S(I,J)=0: D1(I)=0: S1(I)=100: Q(I)=0: B(I)=6000
160    NEXT J
170   NEXT I
180 L$=".+*ABCDE" ' Map characters
190 M$="ABCDEFGHIJKL" ' Map column names
195 PRINT "              **********   STAR TRADERS   **********":PRINT
197 PRINT:PRINT
200 INPUT "TYPE A 3 DIGIT NUMBER  ";R1
220 R1=RND(-R1/1000)
230   FOR I=1 TO 9
240    FOR J=1 TO 12
250 IF INT(20*RND(R1)+1)<>10 THEN M(I,J)=1 ELSE M(I,J)=3
260    NEXT J
270   NEXT I
280 PRINT Z1$
290 INPUT "HOW MANY PLAYERS  (1-4)  ";P1
300 PRINT:PRINT
310 PRINT "DOES ANY PLAYER NEED INSTRUCTIONS  "
315 INPUT "(TYPE 0 FOR NO)";Q
320 IF Q<>0 THEN GOSUB 3410
330 PRINT Z1$
340   FOR I=1 TO P1
350 PRINT "PLAYER",I,
370 INPUT " WHAT IS YOUR NAME  ";P$
380 IF I=1 THEN P1$=P$
390 IF I=2 THEN P2$=P$
400 IF I=3 THEN P3$=P$
410 IF I=4 THEN P4$=P$
420   NEXT I
430 PRINT Z1$
440 PRINT TAB(10),"...NOW I WILL DECIDE WHO GOES FIRST...":PRINT:PRINT
445 PRINT:PRINT
450 PRINT:PRINT:PRINT
460 I=INT(P1*RND(R1)+1)
470 GOSUB 490
480 GOTO 550
490 PRINT
500 ON I GOTO 510,520,530,540: REM - IDENTIFY PLAYER
510 PRINT P1$;:P5$=P1$: RETURN
520 PRINT P2$;:P5$=P2$: RETURN
530 PRINT P3$;:P5$=P3$: RETURN
540 PRINT P4$;:P5$=P4$: RETURN
550 PRINT " IS THE FIRST PLAYER TO MOVE."
560 FOR W=1 TO 2000: NEXT
570 K=0
580 P=I:GOTO 610
590 K=K+1: IF K=48 THEN 4450
600 P=P+1: IF P=P1+1 THEN P=1
610  FOR I=1 TO 5: REM   SELECT 5 LEGAL MOVES
620 R(I)=INT(9*RND(R1)+1)
630 C(I)=INT(12*RND(R1)+1)
640    FOR I1=I-1 TO 0 STEP -1
650 IF R(I)=R(I1) AND C(I)=C(I1) THEN  620 'IF space already in list try again
660    NEXT I1
670 IF M(R(I),C(I))>1 THEN 620 'If space occupied try again
680    FOR I1=1 TO 5
690 IF Q(I1)=0 THEN  870 'IF Q(I1) is 0 accept postion TODO: Q 
700    NEXT I1
710 IF M(R(I),C(I)+1)>3 THEN 870 'If current location is next to company accept
720 IF M(R(I),C(I)-1)>3 THEN 870
730 IF M(R(I)+1,C(I))>3 THEN 870
740 IF M(R(I)-1,C(I))>3 THEN 870
' If one neighbor is a star or a selected space
' and the other neighbors are not a company then try again
750 A1=M(R(I),C(I)+1)
760 A2=M(R(I),C(I)-1)
770 A3=M(R(I)+1,C(I))
780 A4=M(R(I)-1,C(I))
790 IF A1=2 AND A2<4 AND A3<4 AND A4<4 THEN  620
800 IF A2=2 AND A1<4 AND A3<4 AND A4<4 THEN  620
810 IF A3=2 AND A1<4 AND A2<4 AND A4<4 THEN  620
820 IF A4=2 AND A1<4 AND A2<4 AND A3<4 THEN  620
830 IF A1=3 AND A2<4 AND A3<4 AND A4<4 THEN  620
840 IF A2=3 AND A1<4 AND A3<4 AND A4<4 THEN  620
850 IF A3=3 AND A1<4 AND A2<4 AND A4<4 THEN  620
860 IF A4=3 AND A1<4 AND A2<4 AND A3<4 THEN  620
870   NEXT I
880 GOSUB 2060
890 PRINT
900 I=P
910 GOSUB 490
920 PRINT ", HERE ARE YOUR LEGAL MOVES FOR THIS TURN"
930 PRINT
940   FOR I=1 TO 5
950 PRINT R(I);MID$(M$,C(I),1);" ";
960   NEXT I
970 PRINT:PRINT
980 INPUT "WHAT IS YOUR MOVE ";R$
990 IF LEN(R$)=0 THEN R$="S"
1000 IF LEFT$(R$,1)="M" THEN R$="" ELSE 1030
1010 GOSUB 2060
1020 GOTO 900
1030 IF LEFT$(R$,1)="S" THEN R$="" ELSE 1060
1040 GOSUB 3230
1050 GOTO 900
1060 IF LEN(R$)<>2 THEN 1110
1070 IF ASC(MID$(R$,2,1))-64<1 THEN 1110
1080 IF ASC(MID$(R$,2,1))-64>12 THEN 1110 ELSE 1120
1090 IF VAL(R$)<1 THEN 1110
1100 IF VAL(R$)>9 THEN 1110
1110 PRINT "I DIDN'T UNDERSTAND THAT - TRY AGAIN ": GOTO 980
1120 R=VAL(LEFT$(R$,1))
1130 C=ASC(RIGHT$(R$,1))-64
1140   FOR I= 1 TO 5
1150 IF R=R(I) AND C=C(I) THEN  1190
1160   NEXT I
1170 PRINT "THAT SPACE WAS NOT INCLUDED IN THE LIST..."
1180 GOTO 980
1190 A1=M(R-1,C)
1200 A2=M(R+1,C)
1210 A3=M(R,C+1)
1220 A4=M(R,C-1)
1230 IF A1<=1 AND A2<=1 AND A3<=1 AND A4<=1 THEN M(R,C)=2 ELSE 1250
1240 GOTO 1760
1250 IF A1>3 AND A2>3 AND A2<>A1 THEN GOSUB 2220:REM - LINE 2220 IS
1260 IF A1>3 AND A3>3 AND A3<>A1 THEN GOSUB 2220:REM   THE MERGER SUB
1270 IF A1>3 AND A4>3 AND A4<>A1 THEN GOSUB 2220
1280 IF A2>3 AND A3>3 AND A3<>A2 THEN GOSUB 2220
1290 IF A2>3 AND A4>3 AND A4<>A2 THEN GOSUB 2220
1300 IF A3>3 AND A4>3 AND A4<>A3 THEN GOSUB 2220
1310 IF A1<4 AND A2<4 AND A3<4 AND A4<4 THEN 1410
1320 IF M(R,C)>3 THEN 1760
1330 IF A1>3 THEN I=A1-3
1340 IF A2>3 THEN I=A2-3
1350 IF A3>3 THEN I=A3-3
1360 IF A4>3 THEN I=A4-3
1370 Q(I)=Q(I)+1
1380 S1(I)=S1(I)+100
1390 M(R,C)=I+3
1400 GOTO 1570
1410   FOR I=1 TO 5
1420 IF Q(I)=0 THEN 1460
1430   NEXT I
1440 IF M(R,C)<3 THEN M(R,C)=2
1450 GOTO 1760
1460 PRINT Z1$
1470 GOSUB 3370
1480 PRINT "A NEW SHIPPING COMPANY HAS BEEN FORMED !"
1490 PRINT "IT'S NAME IS ",
1500 RESTORE
1510 READ N,C$
1520 IF I<>N THEN 1510
1530 PRINT C$
1540 S(I,P)=S(I,P)+5
1550 Q(I)=1
1560 PRINT:PRINT:PRINT:PRINT
1570 IF A1=3 THEN S1(I)=S1(I)+500
1580 IF A2=3 THEN S1(I)=S1(I)+500
1590 IF A3=3 THEN S1(I)=S1(I)+500
1600 IF A4=3 THEN S1(I)=S1(I)+500
1610 IF A1=2 THEN S1(I)=S1(I)+100 ELSE 1640
1620 Q(I)=Q(I)+1
1630 M(R-1,C)=I+3
1640 IF A2=2 THEN S1(I)=S1(I)+100 ELSE 1670
1650 Q(I)=Q(I)+1
1660 M(R+1,C)=I+3
1670 IF A3=2 THEN S1(I)=S1(I)+100 ELSE 1700
1680 Q(I)=Q(I)+1
1690 M(R,C+1)=I+3
1700 IF A4=2 THEN S1(I)=S1(I)+100 ELSE 1730
1710 Q(I)=Q(I)+1
1720 M(R,C-1)=I+3
1730 IF S1(I)>=3000 THEN T1=I ELSE 1750
1740 GOSUB 3100
1750 M(R,C)=I+3
1760   FOR I=1 TO 5
1770 B(P)=B(P)+INT(.05*S(I,P)*S1(I))
1780   NEXT I
1790   FOR I=1 TO 5
1800 IF Q(I)=0 THEN 2040
1810 PRINT:PRINT   "YOUR CURRENT CASH= $";B(P)
1820 PRINT:PRINT   "BUY HOW MANY SHARES OF ";
1830 RESTORE
1840 READ N,C$
1850 IF I<>N THEN 1840
1860 PRINT C$;
1870 PRINT " AT $";S1(I)
1880 PRINT TAB(5); "YOU NOW OWN ";S(I,P);
1890 INPUT R3$:IF LEN(R3$)=0 THEN R3$="0"
1900 IF R3$(1,1)="M" THEN R3$="" ELSE 1930
1910 GOSUB 2060
1920 GOTO 1810
1930 IF R3$(1,1)="S" THEN R3$="" ELSE 1960
1940 GOSUB 3230
1950 GOTO 1810
1960 R3=VAL(R3$)
1970 R3$=""
1980 IF R3*S1(I)<=B(P) THEN 2010
1990 PRINT "YOU ONLY HAVE $";B(P);" - TRY AGAIN"
2000 GOTO 1810
2010 IF R3=0 THEN 2040
2020 S(I,P)=S(I,P)+R3
2030 B(P)=B(P)-(R3*S1(I))
2040   NEXT I
2050 GOTO 590
2060 PRINT Z1$: REM             SUBROUTINE - PRINT MAP
2070 PRINT TAB(22);"MAP OF THE GALAXY"
2080 PRINT TAB(21);"*******************"
2090 PRINT TAB(13);" A  B  C  D  E  F  G  H  I  J  K  L"
2100   FOR R2=1 TO 9
2110 PRINT "         ";R2;" ";
2120 FOR C2=1 TO 12
2130 PRINT " ";
2140 Z2=M(R2,C2)
2150 IF Z2=0 THEN Z2=Z2+1
2160 PRINT MID$(L$,Z2,1);" ";
2180 NEXT
2190 PRINT
2200 NEXT
2210 RETURN
2220 F1=A1-3: IF F1<0 THEN F1=0:REM  SUBROUTINE - CALCULATES THE
2230 F2=A2-3: IF F2<0 THEN F2=0:REM     SURVIVOR IN THE EVENT
2240 F3=A3-3: IF F3<0 THEN F3=0:REM        OF A MERGER
2250 F4=A4-3: IF F4<0 THEN F4=0
2260 T=Q(F1)
2270 T1=F1
2280 IF Q(F2)>Q(F1) THEN T=Q(F2) ELSE 2300
2290 T1=F2
2300 IF Q(F3)>T THEN T=Q(F3) ELSE 2320
2310 T1=F3
2320 IF Q(F4)>T THEN T=Q(F4) ELSE 2340
2330 T1=F4
2340 IF F1=T1 OR A1<4 THEN 2370
2350 X=F1
2360 GOSUB 2470
2370 IF F2=T1 OR A2<4 THEN 2400
2380 X=F2
2390 GOSUB 2470
2400 IF F3=T1 OR A3<4 THEN 2430
2410 X=F3
2420 GOSUB 2470
2430 IF F4=T1 OR A4<4 THEN 2460
2440 X=F4
2450 GOSUB 2470
2460 RETURN
2470 PRINT Z1$
2480 GOSUB 3370: REM     SUBROUTINE - PERFORMS CALCULATIONS
2490 RESTORE: REM         TO ACCOMPLISH A MERGER
2500 READ N,C$
2510 IF X<>N THEN 2500
2520 C1$=C$
2530 PRINT C1$;
2540 PRINT " HAS JUST BEEN MERGED INTO ";
2550 RESTORE
2560 READ N,C$
2570 IF T1<>N THEN 2560
2580 C2$=C$
2590 PRINT C2$;"!"
2610 PRINT "PLEASE NOTE THE FOLLOWING TRANSACTIONS."
2620 PRINT
2630 PRINT TAB(3);"OLD STOCK = ";C1$;"      NEW STOCK = ";
2640 PRINT C2$
2650 PRINT
2660 PRINT "PLAYER";TAB(10);"OLD STOCK";TAB(22);"NEW STOCK";
2670 PRINT TAB(34);"TOTAL HOLDINGS";TAB(53);"BONUS PAID"
2680   FOR I=1 TO P1
2690 GOSUB 490
2700 PRINT TAB(10);S(X,I);TAB(22);INT((.5*S(X,I))+.5);
2710 PRINT TAB(34);S(T1,I)+INT((.5*S(X,I))+.5);
2720 X1=0
2730    FOR I1=1 TO P1
2740 X1=X1+S(X,I1)
2750    NEXT
2760 PRINT TAB(53);" $";INT(10*((S(X,I)/X1)*S1(X)))
2770   NEXT I
2780   FOR I=1 TO P1
2790 S(T1,I)=S(T1,I)+INT((.5*S(X,I))+.5)
2800 B(I)=B(I)+INT(10*((S(X,I)/X1)*S1(X)))
2810   NEXT I
2820   FOR I=1 TO 9
2830    FOR J=1 TO 12
2840 IF M(I,J)=X+3 THEN M(I,J)=T1+3
2850    NEXT J
2860   NEXT I
2870 A1=M(R-1,C)
2880 A2=M(R+1,C)
2890 A3=M(R,C+1)
2900 A4=M(R,C-1)
2910 F1=A3-3
2920 IF F1<0 THEN F1=0
2930 F2=A2-3
2940 IF F2<0 THEN F2=0
2950 Q(T1)=Q(T1)+Q(X)
2960 S1(T1)=S1(T1)+S1(X)
2970 IF S1(T1)=>3000 THEN GOSUB 3100
2980 F3=A3-3
2990 IF F3<0 THEN F3=0
3000 F4=A4-3
3010 IF F4<0 THEN F4=0
3020 S1(X)=100
3030 Q(X)=0
3040   FOR I=1 TO P1
3050 S(X,I)=0
3060   NEXT I
3070 PRINT:PRINT
3080 M(R,C)=T1+3
3090 RETURN
3100 GOSUB 3370: REM        SUBROUTINE - CALCULATES STOCK SPLITS
3110 PRINT "THE STOCK OF ",
3120 RESTORE
3130 READ N,C$
3140 IF T1<>N THEN 3130
3150 PRINT C$;" HAS SPLIT 2 FOR 1 !"
3170 S1(T1)=INT(S1(T1)/2)
3190   FOR I1=1 TO P1
3200 S(T1,I1)=2*S(T1,I1)
3210   NEXT I1
3220 RETURN
3230 PRINT Z1$
3240 PRINT
3250 PRINT "STOCK";TAB(30);"PRICE PER SHARE";
3260 PRINT TAB(50);"YOUR HOLDINGS"
3270   FOR I3=1 TO 5
3280 IF S1(I3)=100 THEN 3340
3290 RESTORE
3300 READ N,C$
3310 IF I3<>N THEN 3300
3320 PRINT C$;
3330 PRINT TAB(30);S1(I3);TAB(50);S(I3,P)
3340   NEXT I3
3350 RESTORE
3360 RETURN
3370 PRINT CHR$(7)
3380 PRINT TAB(22);"SPECIAL ANNOUNCEMENT !!!":PRINT
3390 PRINT
3400 RETURN
3410 PRINT Z1$
3420 PRINT "   STAR LANES IS A GAME OF INTERSTELLAR TRADING."
3430 PRINT "THE OBJECT OF THE GAME IS TO AMASS THE GREATEST AMOUNT"
3440 PRINT "OF MONEY.  THIS IS ACCOMPLISHED BY ESTABLISHING VAST,"
3450 PRINT "INTERSTELLAR SHIPPING LANES, AND PURCHASING STOCK IN"
3460 PRINT "THE COMPANIES THAT CONTROL THOSE TRADE ROUTES.  DURING"
3470 PRINT "THE COURSE OF THE GAME, STOCK APPRECIATES IN VALUE AS"
3480 PRINT "THE SHIPPING COMPANIES BECOME LARGER.  ALSO, SMALLER"
3490 PRINT "COMPANIES CAN BE MERGED INTO LARGER ONES, AND STOCK"
3500 PRINT "IN THE SMALLER FIRM IS CONVERTED INTO STOCK IN THE"
3510 PRINT "LARGER ONE AS DESCRIBED BELOW.":PRINT
3520 PRINT "EACH TURN, THE COMPUTER WILL PRESENT THE PLAYER WITH"
3530 PRINT "FIVE PROSPECTIVE SPACES TO OCCUPY ON A 9X12 MATRIX"
3540 PRINT "(ROWS 1-9, COLUMNS A-L).  THE PLAYER, AFTER EXAMINING"
3550 PRINT "THE MAP OF THE GALAXY TO DECIDE WHICH SPACE HE WISHES"
3560 PRINT "TO OCCUPY, RESPONDS WITH THE ROW AND COLUMN OF THAT"
3570 PRINT "SPACE, I.E., 7E, 8A, ETC.  THERE ARE FOUR POSSIBLE"
3580 PRINT "MOVES A PLAYER CAN MAKE.":PRINT:PRINT
3590 PRINT
3600 GOSUB 3620
3610 GOTO 3660
3620 INPUT "PRESS RETURN TO CONTINUE";X2$
3650 RETURN
3660 PRINT Z1$
3670 PRINT "   1. HE CAN ESTABLISH AN UNATTACHED OUTPOST- IF HE"
3680 PRINT "SELECTS A SPACE THAT IS NOT ADJACENT TO A STAR, ANOTHER"
3690 PRINT "UNATTACHED OUTPOST, OR AN EXISTING SHIPPING LANE, THIS"
3700 PRINT "SPACE WILL BE DESIGNATED WITH A '+'.  HE WILL THEN PROCEED"
3710 PRINT "WITH STOCK TRANSACTIONS, AS LISTED BELOW.":PRINT
3720 PRINT "   2. HE CAN ADD TO AN EXISTING LANE- IF HE SELECTS A"
3730 PRINT "SPACE THAT IS ADJACENT TO ONE - AND ONLY ONE EXISTING"
3740 PRINT "SHIPPING LANE, THE SPACE HE SELECTS WILL BE ADDED TO"
3750 PRINT "THAT SHIPPING LANE AND WILL BE DESIGNATED WITH THE FIRST"
3760 PRINT "LETTER OF THE COMPANY THAT OWNS THAT LANE.  IF THERE ARE"
3770 PRINT "ANY STARS OR UNATTACHED OUTPOSTS ALSO ADJACENT TO THE"
3780 PRINT "SELECTED SPACE, THEY, TOO, WILL BE INCORPORATED INTO THE"
3790 PRINT "EXISTING LANE.  EACH NEW SQUARE ADJACENT TO A STAR ADDS"
3800 PRINT "$500 PER SHARE, AND EACH NEW OUTPOST ADDS $100 PER SHARE"
3810 PRINT "TO THE MARKET VALUE OF THE STOCK OF THAT COMPANY."
3820 PRINT:PRINT
3830 GOSUB 3620
3840 PRINT Z1$
3850 PRINT "   3. HE MAY ESTABLISH A NEW SHIPPING LANE- IF THERE"
3860 PRINT "ARE FIVE OR LESS EXISTING SHIPPING LANES ESTABLISHED,"
3870 PRINT "THE PLAYER MAY, GIVEN THE PROPER SPACE TO PLAY, ESTABLISH"
3880 PRINT "A NEW SHIPPING LANE.  HE MAY DO THIS BY OCCUPYING A SPACE"
3890 PRINT "ADJACENT TO A STAR OR ANOTHER UNATTACHED OUTPOST, BUT"
3900 PRINT "NOT ADJACENT TO AN EXISTING SHIPPING LANE.  IF HE"
3910 PRINT "ESTABLISHES A NEW SHIPPING LANE, HE IS AUTOMATICALLY"
3920 PRINT "ISSUED 5 SHARES IN THE NEW COMPANY AS A REWARD.  HE"
3930 PRINT "MAY THEN PROCEED TO BUY STOCK IN ANY ACTIVE COMPANY,"
3940 PRINT "INCLUDING THE ONE JUST FORMED, AS DESCRIBED BELOW."
3950 PRINT "THE MARKET VALUE OF THE NEW STOCK IS ESTABLISHED BY"
3960 PRINT "THE NUMBER OF STARS AND OCCUPIED SPACES AS DESCRIBED"
3970 PRINT "IN #2 ABOVE."
3980 PRINT:PRINT
3990 GOSUB 3620
4000 PRINT Z1$
4010 PRINT "   4. HE MAY MERGE TWO EXISTING COMPANIES- IF A PLAYER"
4020 PRINT "SELECTS A SPACE ADJACENT TO TWO EXISTING SHIPPING"
4030 PRINT "LANES, A MERGER OCCURS.  THE LARGER COMPANY TAKES OVER THE"
4040 PRINT "SMALLER COMPANY - (IF BOTH COMPANIES ARE THE SAME SIZE"
4050 PRINT "PRIOR TO THE MERGER, THEN THE SURVIVOR IS DETERMINED BY"
4060 PRINT "ALPHABETICAL ORDER OF THE TWO COMPANY NAMES - THE EARLIER"
4070 PRINT "SURVIVES).  THE STOCK OF THE SURVIVING COMPANY IS"
4080 PRINT "INCREASED IN VALUE ACCORDING TO THE NUMBER OF SPACES"
4090 PRINT "AND STARS ADDED TO ITS LANE.  EACH PLAYERS STOCK IN"
4100 PRINT "THE DEFUNCT COMPANY IS EXCHANGED FOR SHARES IN THE"
4110 PRINT "SURVIVOR ON A RATIO OF 2 FOR 1.  ALSO, EACH PLAYER"
4120 PRINT "IS PAID A CASH BONUS PROPORTIONAL TO THE PERCENTAGE"
4130 PRINT "OF OUTSTANDING STOCK HE HELD IN THE DEFUNCT COMPANY."
4140 PRINT "NOTE: AFTER A COMPANY BECOMES DEFUNCT THROUGH THE"
4150 PRINT "MERGER PROCESS, IT CAN REAPPEAR ELSE WHERE ON THE"
4160 PRINT "BOARD WHEN, AND IF, A NEW COMPANY IS ESTABLISHED."
4170 PRINT:PRINT
4180 GOSUB 3620
4190 PRINT Z1$
4200 PRINT "   NEXT THE COMPUTER ADDS STOCK DIVIDENDS TO THE PLAYER'S"
4210 PRINT "CASH ON HAND (5% OF THE MARKET VALUE OF THE STOCK IN HIS"
4220 PRINT "POSSESSION), AND OFFERS HIM THE OPPORTUNITY TO PURCHASE"
4230 PRINT "STOCK IN ANY OF THE ACTIVE COMPANIES ON THE BOARD."
4240 PRINT "STOCK MAY NOT BE SOLD, BUT THE MARKET VALUE OF EACH"
4250 PRINT "PLAYER'S STOCK IS TAKEN INTO ACCOUNT AT THE END OF THE"
4260 PRINT "GAME TO DETERMINE THE WINNER.  IF THE MARKET VALUE OF A GIVEN"
4270 PRINT "STOCK EXCEEDS $3000 AT ANY TIME DURING THE GAME, THAT"
4280 PRINT "STOCK SPLITS 2 FOR 1.  THE PRICE IS CUT IN HALF, AND "
4290 PRINT "THE NUMBER OF SHARES OWNED BY EACH PLAYER IS DOUBLED."
4300 PRINT
4310 PRINT "NOTE:  THE PLAYER MAY LOOK AT HIS PORTFOLIO AT ANY TIME"
4320 PRINT "DURING THE COURSE OF HIS TURN BY RESPONDING WITH 'STOCK'"
4330 PRINT "TO AN INPUT STATEMENT.  LIKEWISE, HE CAN REVIEW THE MAP"
4340 PRINT "OF THE GALAXY BY TYPING 'MAP' TO AN INPUT STATEMENT."
4350 PRINT:PRINT
4360 GOSUB 3620
4370 PRINT Z1$
4380 PRINT:PRINT:PRINT:PRINT:PRINT:PRINT:PRINT:PRINT
4390 PRINT TAB(16);"** GAME ENDS AFTER 48 MOVES **"
4400 PRINT:PRINT:PRINT:PRINT
4410 PRINT "PLAYER WITH THE GREATEST NET WORTH AT THAT POINT IS THE WINNER."
4420 PRINT:PRINT
4430 FOR W=1 TO 2000:NEXT W
4440 RETURN
4450 PRINT Z1$
4460 GOSUB 3370
4470 FOR W=1 TO 500:NEXT W:PRINT CHR$(7)
4480 FOR W=1 TO 500:NEXT W:PRINT CHR$(7)
4490 PRINT TAB(10);" THE GAME IS OVER - HERE ARE THE FINAL STANDINGS"
4500 PRINT:PRINT:PRINT:PRINT
4510 PRINT CHR$(7)
4520 PRINT "PLAYER";TAB(10);"CASH VALUE OF STOCK";TAB(33);"CASH ON HAND";
4530 PRINT TAB(50);"NET WORTH"
4540 PRINT
4550   FOR I=1 TO P1
4560    FOR J=1 TO 5
4570 D1(I)=D1(I)+(S1(J)*S(J,I))
4580    NEXT J
4590   NEXT I
4600   FOR I=1 TO P1
4610 GOSUB 490
4620 PRINT TAB(10);"$";D1(I);TAB(33);"$";B(I);
4630 PRINT TAB(50);"$";D1(I)+B(I)
4640   NEXT I
4650 PRINT:PRINT:PRINT:PRINT
4660 END
