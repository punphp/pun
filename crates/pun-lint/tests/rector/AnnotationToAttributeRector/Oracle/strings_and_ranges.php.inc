<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

final class Methods
{
    /**
     * @ORM\Column(nullable="true")
     */
    public function run1()
    {
    }

    /**
     * @ORM\Column(nullable="false", name="x")
     */
    public function run2()
    {
    }

    /**
     * @When(a..z) text
     */
    public function run3()
    {
    }

    /**
     * @When(b..y)
     */
    public function run4()
    {
    }

    /**
     * @When b..y some
     */
    public function run5()
    {
    }

    /**
     * @When() 
     */
    public function run6()
    {
    }

    /**
     * @When()
     */
    public function run7()
    {
    }
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

final class Methods
{
    #[ORM\Column(nullable: true)]
    public function run1()
    {
    }

    #[ORM\Column(nullable: false, name: 'x')]
    public function run2()
    {
    }

    /**
     * ..z) text
     */
    #[\Behat\Step\When('(a)')]
    public function run3()
    {
    }

    /**
     * ..y)
     */
    #[\Behat\Step\When('(b)')]
    public function run4()
    {
    }

    #[\Behat\Step\When('b..y some')]
    public function run5()
    {
    }

    #[\Behat\Step\When]
    public function run6()
    {
    }

    #[\Behat\Step\When]
    public function run7()
    {
    }
}
