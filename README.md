#week2

main1:
u32固定字節數, 所有權可被直接默認複製

main2:
不固定字節數的類型，所有權不能被直接默認複製, 只能轉移，如要複製，就需要使用.clone

main3:
可變引用不能被複制，只能被轉移
可變引用具排它性，當被轉移後，之前的擁有者就沒有所有權

main4:
不可變引用可以被複制

main5:
scope的設計很重要，
引用的作用域是從它定義到它最後一次使用時結束，
不可變引用在同一時刻可以同時出現多個，
可變引用在同一時刻只能出現一個。
可變引用與不可變引用的作用域不能交疊（overlap）。

