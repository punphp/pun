<?php

$object = new class {
    public function run()
    {
        return [1, 2];
    }
};
-----
<?php

$object = new class {
    /**
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
};
