<?php
class foo {
	function foo($name) {
     	$GLOBALS['List']= $this;
     	$this->Name = $name;
		$GLOBALS['List']->echoName();
	}

	function echoName() {
     	$GLOBALS['names'][]=$this->Name;
	}
}


$bar1 =new foo('constructor');
$bar1->Name = 'outside';
$bar1->echoName();
$List->echoName();

print ($names==array('constructor','outside','outside')) ? 'success':'failure';
