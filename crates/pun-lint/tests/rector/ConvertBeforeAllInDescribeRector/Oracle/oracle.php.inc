<?php

describe('a', function () {
    beforeAll(fn () => 1);
    afterAll(function () {});
    \beforeAll(function () {});
    BEFOREALL(function () {});
    beforeAll(function () {})->skip();
    $x = beforeAll(function () {});
    if (true) {
        beforeAll(function () {});
    }
    describe('b', function () {
        afterAll(function () {});
        describe('c', fn () => beforeAll(function () {}));
    });
    it('d', function () {
        beforeAll(function () {});
    });
    (beforeAll(function () {}));
    beforeAll  (function () {});
});
describe('e', fn () => beforeAll(function () {}));
DESCRIBE('f', function () { beforeAll(function () {}); });
\describe('g', function () { beforeAll(function () {}); });
describe(function () { beforeAll(function () {}); });
describe('h', static function () { afterAll(function () {}); });
-----
<?php

describe('a', function () {
    beforeEach(fn () => 1);
    afterEach(function () {});
    beforeEach(function () {});
    BEFOREALL(function () {});
    beforeAll(function () {})->skip();
    $x = beforeAll(function () {});
    if (true) {
        beforeAll(function () {});
    }
    describe('b', function () {
        afterEach(function () {});
        describe('c', fn () => beforeAll(function () {}));
    });
    it('d', function () {
        beforeAll(function () {});
    });
    (beforeEach(function () {}));
    beforeEach  (function () {});
});
describe('e', fn () => beforeAll(function () {}));
DESCRIBE('f', function () { beforeAll(function () {}); });
\describe('g', function () { beforeEach(function () {}); });
describe(function () { beforeAll(function () {}); });
describe('h', static function () { afterEach(function () {}); });
