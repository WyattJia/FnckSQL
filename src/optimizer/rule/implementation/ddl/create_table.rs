use crate::errors::DatabaseError;
use crate::optimizer::core::memo::{Expression, GroupExpression};
use crate::optimizer::core::pattern::{Pattern, PatternChildrenPredicate};
use crate::optimizer::core::rule::{ImplementationRule, MatchPattern};
use crate::optimizer::core::statistics_meta::StatisticMetaLoader;
use crate::planner::operator::{Operator, PhysicalOption};
use crate::single_mapping;
use crate::storage::Transaction;
use std::sync::LazyLock;

static CREATE_TABLE_PATTERN: LazyLock<Pattern> = LazyLock::new(|| Pattern {
    predicate: |op| matches!(op, Operator::CreateTable(_)),
    children: PatternChildrenPredicate::None,
});

#[derive(Clone)]
pub struct CreateTableImplementation;

single_mapping!(
    CreateTableImplementation,
    CREATE_TABLE_PATTERN,
    PhysicalOption::CreateTable
);
