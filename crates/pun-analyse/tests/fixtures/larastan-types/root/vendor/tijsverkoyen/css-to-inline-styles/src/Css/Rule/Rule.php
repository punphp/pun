<?php

namespace TijsVerkoyen\CssToInlineStyles\Css\Rule;

use Symfony\Component\CssSelector\Node\Specificity;
use TijsVerkoyen\CssToInlineStyles\Css\Property\Property;

final class Rule
{
    /**
     * @var string
     */
    private $selector;

    /**
     * @var Property[]
     */
    private $properties;

    /**
     * @var Specificity
     */
    private $specificity;

    /**
     * @var integer
     */
    private $order;

    /**
     * Rule constructor.
     *
     * @param string      $selector
     * @param Property[]  $properties
     * @param Specificity $specificity
     * @param int         $order
     */
    public function __construct($selector, array $properties, Specificity $specificity, $order)
    {}

    /**
     * Get selector
     *
     * @return string
     */
    public function getSelector()
    {}

    /**
     * Get properties
     *
     * @return Property[]
     */
    public function getProperties()
    {}

    /**
     * Get specificity
     *
     * @return Specificity
     */
    public function getSpecificity()
    {}

    /**
     * Get order
     *
     * @return int
     */
    public function getOrder()
    {}
}
