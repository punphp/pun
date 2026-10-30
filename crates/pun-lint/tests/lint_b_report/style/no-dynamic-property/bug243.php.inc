<?php

namespace Bug243;

function test(\SimpleXMLElement $xml) {
	$xml->{'foo-bar'};
};
-----
