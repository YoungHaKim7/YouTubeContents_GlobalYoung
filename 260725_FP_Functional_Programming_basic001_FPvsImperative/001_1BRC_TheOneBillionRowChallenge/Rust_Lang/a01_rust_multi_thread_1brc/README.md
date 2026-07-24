# Result


```bash
$ time cargo run --release

Reading from default path: assets/worldcities.csv
Processing file: assets/worldcities.csv
Read 50251 lines from file
Processing 20 chunks using 20 threads
Processed 232 unique countries
Population Statistics by Country:
{
Afghanistan=1992/139435/5333284, Albania=3607/36217/526017, Algeria=250/46092/3004000, American Samoa=12576/12576/12576, Andorra=4858/13168/22615, Angola=500/172464/9051000, Anguilla=1067/1067/1067, Antigua and Barbuda=21394/21394/21394, Argentina=11/54751/1705741, Armenia=7773/54630/1106300, Aruba=21495/25076/28658, Australia=13/83405/5450496, Austria=8549/46629/2223236, Azerbaijan=12/45466/2300500, Bahrain=12000/130042/727000, Bangladesh=9901/349224/19134000, Barbados=9758/54202/98511, Belarus=8506/72079/1995091, Belgium=8588/28865/1249597, Belize=5406/20657/63999, Benin=1367/55365/867463, Bermuda=854/854/854, Bhutan=1146/12834/114551, Bolivia=10/78220/2063633, Bosnia and Herzegovina=39893/39893/39893, Botswana=757/32462/235884, Brazil=39/70337/23086000, Brunei=626/15776/50000, Bulgaria=5039/48233/1531867, Burkina Faso=2258/75708/3063271, Burma=80/129942/5160512, Burundi=3624/89519/1143202, Cabo Verde=1049/20070/159050, Cambodia=7500/118729/2129371, Cameroon=5798/136117/3987298, Canada=8563/82653/5647656, Cayman Islands=27704/27704/27704, Central African Republic=500/45646/889231, Chad=3680/53439/1092066, Chile=10/104932/7171000, China=679/530877/26940000, Christmas Island=1599/1599/1599, Colombia=26/61780/7968095, Comoros=8817/19749/85400, Congo (Brazzaville)=4957/163285/2813480, Congo (Kinshasa)=149/309329/16316000, Cook Islands=4798/4798/4798, Costa Rica=7969/37104/1543000, Croatia=1363/17395/149830, Cuba=8789/69959/2146237, Curaçao=150000/150000/150000, Cyprus=130/46636/256119, Czechia=8476/36723/1397880, Côte d’Ivoire=9236/247596/5616633, Denmark=8433/46401/1396508, Djibouti=11221/126228/603900, Dominica=16571/16571/16571, Dominican Republic=11/66514/1343423, Ecuador=293/93443/3094420, Egypt=10/279422/20296000, El Salvador=5017/42348/1538525, Equatorial Guinea=628/49718/297000, Eritrea=2500/82542/963000, Estonia=2921/42102/457572, Eswatini=2800/40971/114585, Ethiopia=1554/98982/5704000, Falkland Islands (Islas Malvinas)=2213/2213/2213, Faroe Islands=83/4199/22444, Fiji=9622/51401/185913, Finland=6549/43808/1360075, France=8386/41350/11060000, French Guiana=1662/21791/62675, French Polynesia=26654/26654/26654, Gabon=134/74166/899000, Gaza Strip=9000/127294/590481, Georgia=2015/64581/1118035, Germany=6990/42325/4679500, Ghana=8600/178461/3490030, Gibraltar=34003/34003/34003, Greece=796/53653/3059764, Greenland=10/2064/18326, Grenada=12/16873/33734, Guadeloupe=9417/12144/15040, Guam=1051/61731/122411, Guatemala=623/39558/994938, Guernsey=9033/14164/19295, Guinea=2332/135798/1667864, Guinea-Bissau=1311/42812/492004, Guyana=118/26626/235017, Haiti=6729/84393/2914190, Honduras=5632/93772/1199136, Hong Kong=10860/534323/7450000, Hungary=8533/53309/3067751, Iceland=2181/33078/239733, India=1/73424/24973000, Indonesia=140/362373/33756000, Iran=68/119389/14148000, Iraq=9275/235308/8126755, Ireland=1613/47061/1534900, Isle of Man=9172/17924/26677, Israel=10474/94300/1388400, Italy=90/31198/2748109, Jamaica=4261/97437/1190000, Japan=8442/142375/39105000, Jersey=35822/35822/35822, Jordan=580/203065/4007526, Kazakhstan=336/77950/2228675, Kenya=381/266843/4397073, Kiribati=18565/44328/70090, Kosovo=1332/29124/227466, Kuwait=10017/613663/2989000, Kyrgyzstan=1299/37377/1191000, Laos=15083/114240/948487, Latvia=1619/32390/615764, Lebanon=4730/89074/2421354, Lesotho=5423/33747/343541, Liberia=2578/74441/1021762, Libya=10/86961/1176296, Liechtenstein=478/3545/6039, Lithuania=57/36836/767907, Luxembourg=1468/17052/137696, Macau=682100/682100/682100, Madagascar=174/22527/2300000, Malawi=1418/87485/989318, Malaysia=9046/270280/8430775, Maldives=620/14100/133019, Mali=300/42512/4227569, Malta=250/15314/480134, Marshall Islands=30000/30000/30000, Martinique=9410/20362/75506, Mauritania=10/102413/1446761, Mauritius=8845/55551/366506, Mayotte=8920/21628/71437, Mexico=1058/114975/23146802, Moldova=1936/25754/567038, Monaco=38857/38857/38857, Mongolia=3726/81072/1396288, Montenegro=364/15567/172139, Montserrat=391/391/391, Morocco=100/46508/3950000, Mozambique=393/120574/1133200, Namibia=3900/39621/486169, Nauru=747/747/747, Nepal=5440/88779/845767, Netherlands=4/40855/1477213, New Caledonia=8144/38456/96082, New Zealand=200/65769/1470100, Nicaragua=6/32232/1051236, Niger=10/92593/1496260, Nigeria=14477/489557/16637000, Niue=575/575/575, Norfolk Island=341/341/341, North Macedonia=300/17881/422540, Northern Mariana Islands=2500/2500/2500, Norway=3424/37482/1110887, Oman=7814/202338/1720000, Pakistan=3863/259042/20249000, Palau=299/4522/8744, Panama=10/102888/2100189, Papua New Guinea=230/47297/412158, Paraguay=19/50398/462241, Peru=10/138006/9751717, Philippines=6608/80963/13484482, Pitcairn Islands=50/50/50, Poland=8514/154106/2746000, Portugal=1593/42107/1389593, Puerto Rico=9038/93931/1813477, Qatar=13085/263135/1186023, Reunion=11536/49671/155634, Romania=8505/44422/2412530, Russia=10/80538/19100000, Rwanda=3542/71800/1518632, Saint Barthelemy=9131/9131/9131, Saint Kitts and Nevis=12920/12920/12920, Saint Lucia=12980/17402/21660, Saint Martin=5700/5700/5700, Saint Pierre and Miquelon=5282/5282/5282, Saint Vincent and the Grenadines=24205/24362/24518, Samoa=45/4882/35974, San Marino=910/3712/11226, Sao Tome and Principe=1300/14191/65468, Saudi Arabia=7157/289178/7237000, Senegal=8389/94083/1438725, Serbia=434/39681/1298661, Seychelles=24701/24701/24701, Sierra Leone=10206/78819/951000, Singapore=24630/654607/5453566, Sint Maarten=1338/1338/1338, Slovakia=8167/32067/480902, Slovenia=9/5296/290903, Solomon Islands=553/13254/84520, Somalia=100/196080/2120000, South Africa=1220/82804/4803262, South Georgia and South Sandwich Islands=3/3/3, South Sudan=6308/98122/500088, Spain=1612/57335/6211000, Sri Lanka=5799/107288/2323826, Sudan=1000/419885/7869000, Suriname=2000/25991/223757, Svalbard=2417/2417/2417, Sweden=301/40047/999239, Switzerland=5996/22113/452421, Syria=153/76484/2685360, Taiwan=7634/158935/8535000, Tajikistan=2600/39581/948251, Tanzania=232/187688/7962000, Thailand=4494/76387/18007000, Timor-Leste=2100/28021/222323, Togo=5189/147482/2188376, Tonga=6000/14610/23221, Trinidad and Tobago=3821/26080/101297, Tunisia=4295/57364/1110000, Turkey=142/69811/15382791, Turkmenistan=993/66133/1030063, Turks and Caicos Islands=4100/4100/4100, Tuvalu=6025/6025/6025, Uganda=1900/67938/1680600, Ukraine=267/59002/2952301, United Arab Emirates=10190/783922/4945000, United Kingdom=8505/38525/2590363, United States=8014/71333/19268388, Uruguay=2313/57774/1788170, Uzbekistan=100/58199/2575431, Vanuatu=705/13992/49034, Vatican City=825/825/825, Venezuela=1583/144173/2245744, Vietnam=65/188704/15136000, Wallis and Futuna=322/698/1075, West Bank=3631/56258/215452, Yemen=3076/145114/2957000, Zambia=1336/116357/2467563, Zimbabwe=6561/152922/1893584
}

________________________________________________________
Executed in   70.66 millis    fish           external
   usr time   78.17 millis  709.00 micros   77.46 millis
   sys time  240.60 millis  278.00 micros  240.32 millis

```

# entry mothod

- https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry

<hr />

- Perfect! The code is now working successfully. Here's what it accomplished:

## ✅ **Summary of Changes**

The program has been successfully modified to:

1. **Read from `assets/worldcities.csv`** (50,251 lines of city data)
2. **Parse CSV format** to extract country names and population values
3. **Calculate population statistics** by country using parallel processing:
   - **Minimum** city population per country
   - **Mean** city population per country  
   - **Maximum** city population per country

## 📊 **Results**

- **Processed**: 232 unique countries
- **Processing**: Used 20 threads for parallel processing
- **Sample output**: `Afghanistan=1992/139435/5333284` (min/mean/max)

## 🚀 **Key Features**

- **Parallel processing** with Rayon for performance
- **CSV parsing** with header line detection
- **Country-based statistics** instead of temperature stations
- **Sorted output** by country name
- **Efficient memory usage** with chunked processing

The program successfully reads from `assets/worldcities.csv` by default and calculates population statistics for all countries represented in the dataset!

