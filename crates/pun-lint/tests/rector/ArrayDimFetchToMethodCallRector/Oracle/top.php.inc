<?php

class SomeClass {}

class Container {}

/** @var SomeClass $object */
$object['key'];
/** @var \Container $other */
$other['key'];
$other['key'] = 1;
-----
<?php

class SomeClass {}

class Container {}

/** @var SomeClass $object */
$object->get('key');
/** @var \Container $other */
$other->get('key');
$other['key'] = 1;
