<?php

namespace Rector\Tests\Transform\Rector\ArrayDimFetch\ArrayDimFetchToMethodCallRector\Fixture;

/** @var \SomeClass $object */
$object['key'];
$object['key'] = 42;
isset($object['key']);
unset($object['key']);

?>
-----
<?php

namespace Rector\Tests\Transform\Rector\ArrayDimFetch\ArrayDimFetchToMethodCallRector\Fixture;

/** @var \SomeClass $object */
$object->get('key');
$object->set('key', 42);
$object->has('key');
$object->unset('key');

?>
