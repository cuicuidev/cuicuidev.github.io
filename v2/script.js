var tablinks = document.getElementsByClassName("tab-link");
var tabcontents = document.getElementsByClassName("tab-contents");

function selectTab(tabName) {
	for (tablink of tablinks) {
		tablink.classList.remove("active-link");
	}

	for (tabcontent of tabcontents) {
		tabcontent.classList.remove("active-tab");
	}

	event.currentTarget.classList.add("active-link");
	document.getElementById(tabName).classList.add("active-tab");

}
