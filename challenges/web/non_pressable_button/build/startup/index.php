<!DOCTYPE html>
<html lang="zh">

<head>
	<meta charset="utf-8">
	<meta name="viewport" content="initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
	<title>原神·启动</title>
	<link rel="stylesheet" href="css/style.css">
	<script src="js/startup.js"></script>
</head>

<body>
	<div class="main" id="startup">
		<div class="content" id="startup_company">
			<img src="img/company.jpg" />
		</div>
		<div class="content" id="startup_game">
			<img src="img/game.jpg" />
			<h1><?php echo file_get_contents("/flag"); ?></h1>
			<div id="startup_game_g">
				抵制不良游戏, 拒绝盗版游戏, 注意自我保护, 谨防受骗上当, 适度游戏益脑, 沉迷游戏伤身, 合理安排时间, 享受健康生活。<br>
				审批文号: 国新出审[2020]1407号 ISBN 978-7-498-07852-0 出版单位: 华东师范大学电子音像出版社有限公司<br>
				著作权人: 上海米哈游天命科技有限公司<br>
				本公司积极履行《网络游戏行业防沉迷自律公约》<br>
			</div>
		</div>
		<div class="content" id="startup_warning">
			<div id="startup_warning_main">
				<h1>警告: 比赛前详阅</h1>
				<img src="img/warning_line.jpg" />
				<p>
					有极多数的人在参加一些CTF比赛时可能会突然癫痫发作，这些影像包括Web题目中出现的闪光或图形。 在<br>
					进行CTF比赛时，这些人可能会出现癫痫症状。 甚至连不具有癫痫史的人，也可能在进行CTF比赛时，出现类<br>
					似癫痫症状。 如果您或您的队友有癫痫史，请在进行游戏之前先与医生咨询。 如果您在进行CTF比赛时出现<br>
					以下症状, 包括眼睛疼痛、 视觉异常、 偏头痛、 痉挛或意识障碍 (诸如昏迷) 等, 请立即中止比赛，并且请您于<br>
					再次进行本比赛之前咨询您的医生。<br><br>
					除上述症状外, 当您感到头痛、 头晕眼花、 恶心想吐或类似晕车症状时, 以及当身体的某些部位感到不舒服或<br>
					疼痛时, 请立即中止比赛。 若在中止比赛后, 症状仍没有减退, 请立即寻求医生的诊疗。<br>
				</p>
			</div>
		</div>
	</div>
</body>

</html>
