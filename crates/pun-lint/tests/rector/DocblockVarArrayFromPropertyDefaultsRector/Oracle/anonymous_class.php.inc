<?php

$object = new class {
    public array $items = ['a', 'b'];
};
-----
<?php

$object = new class {
    /**
     * @var string[]
     */
    public array $items = ['a', 'b'];
};
