<!DOCTYPE html>
<html lang="zh-TW">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>陳琬昀簡介</title>

	<style type="text/css">
	* { font-family:"標楷體"; margin-left:auto; margin-right:auto;}
	h1 {color:blue; font-size:45px;}
	h2 {color:#33ff33; font-size:30px;}
	</style>

	<script>
	function change1() {
	document.getElementById("pic").src = "mountain.jpg";
	document.getElementById("h2text").innerText = "靜宜資管";
	}

	function change2() {
	document.getElementById("pic").src = "cliff.jpg";
	document.getElementById("h2text").innerText = "Wan-Yun Chen";
	}
	</script>

</head>

<body>

	<ul>
		<li>關於我</li>
		<li>電影配樂</li>
		<li>推薦影片</li>
	</ul>

	<img src="cliff.jpg" width="110%" id="pic" onmouseover="change1()" onmouseout="change2()"></img>

	<h3>陳琬昀(Wan-Yun Chen)</h3>
	個人網頁：<a href="https://www1.pu.edu.tw/~tcyang">https://www1.pu.edu.tw/~tcyang</a><br>
	FB：<a href="https://www.facebook.com/tcyang1971" target="_blank">https://www.facebook.com/tcyang1971</a><br>
	Tel: <a href="tel:0426328001,18110">04-26328001#18110</a><br>
	E-Mail: <a href="mailto:tcyang@pu.edu.tw">tcyang@pu.edu.tw</a><br>

	<h3>電影配樂</h3>
	大象席地而坐電影配樂<br>
	<audio controls>
		<source src="elephant.mp3" type="audio/mP3">
	</audio><br>

	<h3>推薦影片</h3>
	不要去臺灣<br>
	<iframe src="https://www.youtube.com/embed/pW88QFpHXa8" allowfullscreen></iframe>

</body>


</html>