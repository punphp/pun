<?php

test('a', 'callback');
it('b');
test('c', static fn () => $this->x(), 'extra');
-----
<?php

test('a', 'callback');
it('b');
test('c', fn () => $this->x(), 'extra');
