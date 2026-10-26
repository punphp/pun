<?php

trait Relations
{
    /**
     * @return $this
     */
    public function withoutRelation($relations)
    {
        $model = clone $this;

        foreach ((array) $relations as $relation) {
            $model->unsetRelation($relation);
        }

        return $model;
    }

    public function unsetRelation($relation)
    {
        return $this;
    }
}

final class Model
{
    use Relations;

    public function copy()
    {
        $model = clone $this;

        return $model;
    }
}
-----
<?php

trait Relations
{
    /**
     * @return $this
     */
    public function withoutRelation($relations)
    {
        $model = clone $this;

        foreach ((array) $relations as $relation) {
            $model->unsetRelation($relation);
        }

        return $model;
    }

    public function unsetRelation($relation)
    {
        return $this;
    }
}

final class Model
{
    use Relations;

    public function copy(): self
    {
        $model = clone $this;

        return $model;
    }
}
