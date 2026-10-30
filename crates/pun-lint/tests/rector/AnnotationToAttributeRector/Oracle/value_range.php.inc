<?php

namespace App;

final class Methods
{
    /**
     * @When("a..z") more
     */
    public function run1()
    {
    }

    /**
     * @When("a..z")
     */
    public function run2()
    {
    }
}
-----
<?php

namespace App;

final class Methods
{
    /**
     * ..z")more
     */
    #[\Behat\Step\When('("a..z")')]
    public function run1()
    {
    }

    /**
     * ..z")
     */
    #[\Behat\Step\When('("a..z")')]
    public function run2()
    {
    }
}
