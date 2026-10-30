<?php

namespace App;

use Behat\Step\When;
use Doctrine\ORM\Mapping as ORM;

final class P
{
    /**
     * @When()
     */
    public function run1()
    {
    }

    /**
     * @When("x") text
     */
    public function run2()
    {
    }

    /**
     * @ORM\Column(name="x")
     */
    public $a;

    /**
     * @ORM\Column(nullable=true)
     */
    public $b;
}
-----
<?php

namespace App;

use Behat\Step\When;
use Doctrine\ORM\Mapping as ORM;

final class P
{
    #[\Behat\Step\When]
    public function run1()
    {
    }

    /**
     * text
     */
    #[\Behat\Step\When('("x")')]
    public function run2()
    {
    }

    #[ORM\Column(name: 'x')]
    public $a;

    #[ORM\Column(nullable: true)]
    public $b;
}
