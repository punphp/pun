<?php

namespace TijsVerkoyen\CssToInlineStyles\Css\Property;

use Symfony\Component\CssSelector\Node\Specificity;

final class Property
{
    /**
     * @var string
     */
    private $name;

    /**
     * @var string
     */
    private $value;

    /**
     * @var Specificity|null
     */
    private $originalSpecificity;

    /**
     * Property constructor.
     * @param string           $name
     * @param string           $value
     * @param Specificity|null $specificity
     */
    public function __construct($name, $value, ?Specificity $specificity = null)
    {}

    /**
     * Get name
     *
     * @return string
     */
    public function getName()
    {}

    /**
     * Get value
     *
     * @return string
     */
    public function getValue()
    {}

    /**
     * Get originalSpecificity
     *
     * @return Specificity|null
     */
    public function getOriginalSpecificity()
    {}

    /**
     * Is this property important?
     *
     * @return bool
     */
    public function isImportant()
    {}

    /**
     * Get the textual representation of the property
     *
     * @return string
     */
    public function toString()
    {}
}
