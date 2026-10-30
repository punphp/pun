<?php

namespace App;

final class SimpleTags
{
    /**
     * Some text
     * @OldName1
     * @When first \
     * second
     * third line
     * @Then(a="b") tail
     * @Unknown value
     */
    public function step()
    {
    }

    /** @When inline */
    public function inline()
    {
    }
}
-----
<?php

namespace App;

final class SimpleTags
{
    /**
     * Some text
     * tail
     * @Unknown value
     */
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\NewName1]
    #[\Behat\Step\When('first second')]
    #[\Behat\Step\Then('(a="b")')]
    public function step()
    {
    }

    #[\Behat\Step\When('inline')]
    public function inline()
    {
    }
}
