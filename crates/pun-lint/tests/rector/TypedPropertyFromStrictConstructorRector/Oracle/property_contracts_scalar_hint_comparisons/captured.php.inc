<?php

namespace QualityPropertyScalarHintComparisons;

final class ScalarHintComparisons
{
    /** @var int|bool */
    private $string;
    private $falseOrObject;
    private $trueOrObject;

    public function __construct(string $string, false|\stdClass $falseOrObject, true|\stdClass $trueOrObject)
    {
        $this->string = $string;
        $this->falseOrObject = $falseOrObject;
        $this->trueOrObject = $trueOrObject;
    }
}
-----
<?php

namespace QualityPropertyScalarHintComparisons;

final class ScalarHintComparisons
{
    private string $string;
    private bool|\stdClass $falseOrObject;
    private bool|\stdClass $trueOrObject;

    public function __construct(string $string, false|\stdClass $falseOrObject, true|\stdClass $trueOrObject)
    {
        $this->string = $string;
        $this->falseOrObject = $falseOrObject;
        $this->trueOrObject = $trueOrObject;
    }
}
