<?php

namespace DeepCopy;

use ArrayObject;
use DateInterval;
use DatePeriod;
use DateTimeInterface;
use DateTimeZone;
use DeepCopy\Exception\CloneException;
use DeepCopy\Filter\ChainableFilter;
use DeepCopy\Filter\Filter;
use DeepCopy\Matcher\Matcher;
use DeepCopy\Reflection\ReflectionHelper;
use DeepCopy\TypeFilter\Date\DateIntervalFilter;
use DeepCopy\TypeFilter\Date\DatePeriodFilter;
use DeepCopy\TypeFilter\Spl\ArrayObjectFilter;
use DeepCopy\TypeFilter\Spl\SplDoublyLinkedListFilter;
use DeepCopy\TypeFilter\TypeFilter;
use DeepCopy\TypeMatcher\TypeMatcher;
use ReflectionObject;
use ReflectionProperty;
use SplDoublyLinkedList;
use WeakMap;

/**
 * @final
 */
class DeepCopy
{
    /**
     * @var WeakMap<object, object> Map of source objects to their copies.
     */
    private $objectMap;

    /**
     * Filters to apply.
     *
     * @var array Array of ['filter' => Filter, 'matcher' => Matcher] pairs.
     */
    private $filters = [];

    /**
     * Type Filters to apply.
     *
     * @var array Array of ['filter' => Filter, 'matcher' => Matcher] pairs.
     */
    private $typeFilters = [];

    /**
     * @var bool
     */
    private $skipUncloneable = false;

    /**
     * @var bool
     */
    private $useCloneMethod;

    /**
     * @param bool $useCloneMethod   If set to true, when an object implements the __clone() function, it will be used
     *                               instead of the regular deep cloning.
     */
    public function __construct($useCloneMethod = false)
    {}

    /**
     * If enabled, will not throw an exception when coming across an uncloneable property.
     *
     * @param $skipUncloneable
     *
     * @return $this
     */
    public function skipUncloneable($skipUncloneable = true)
    {}

    /**
     * Deep copies the given object.
     *
     * @template TObject
     *
     * @param TObject $object
     *
     * @return TObject
     */
    public function copy($object)
    {}

    public function addFilter(Filter $filter, Matcher $matcher)
    {}

    public function prependFilter(Filter $filter, Matcher $matcher)
    {}

    public function addTypeFilter(TypeFilter $filter, TypeMatcher $matcher)
    {}

    public function prependTypeFilter(TypeFilter $filter, TypeMatcher $matcher)
    {}

    private function recursiveCopy($var)
    {}

    /**
     * Copy an array
     * @param array $array
     * @return array
     */
    private function copyArray(array $array)
    {}

    /**
     * Copies an object.
     *
     * @param object $object
     *
     * @throws CloneException
     *
     * @return object
     */
    private function copyObject($object)
    {}

    private function copyObjectProperty($object, ReflectionProperty $property)
    {}

    /**
     * Returns first filter that matches variable, `null` if no such filter found.
     *
     * @param array $filterRecords Associative array with 2 members: 'filter' with value of type {@see TypeFilter} and
     *                             'matcher' with value of type {@see TypeMatcher}
     * @param mixed $var
     *
     * @return TypeFilter|null
     */
    private function getFirstMatchedTypeFilter(array $filterRecords, $var)
    {}

    /**
     * Returns first element that matches predicate, `null` if no such element found.
     *
     * @param array    $elements Array of ['filter' => Filter, 'matcher' => Matcher] pairs.
     * @param callable $predicate Predicate arguments are: element.
     *
     * @return array|null Associative array with 2 members: 'filter' with value of type {@see TypeFilter} and 'matcher'
     *                    with value of type {@see TypeMatcher} or `null`.
     */
    private function first(array $elements, callable $predicate)
    {}
}
