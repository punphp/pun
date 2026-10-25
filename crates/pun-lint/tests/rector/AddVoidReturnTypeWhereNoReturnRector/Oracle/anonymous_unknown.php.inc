<?php

$instance = new class extends \Unknown\Base {
    public function run()
    {
        echo 'run';
    }
};
-----
<?php

$instance = new class extends \Unknown\Base {
    public function run(): void
    {
        echo 'run';
    }
};
