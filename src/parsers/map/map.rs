use super::mapparser::MapParser;
use crate::parser::parser::Parser;

pub trait Map<TInput, TOutput>
where
    TInput: Send + Sync,
    TOutput: Send + Sync,
    Self: Parser<Output = TInput>,
{
    fn map<TKind, FKindMapper, FValueMapper>(
        self,
        kind_mapper: FKindMapper,
        value_mapper: FValueMapper,
    ) -> MapParser<TInput, Self, TOutput, FValueMapper>
    where
        TKind: Into<String>,
        FKindMapper: FnOnce(&str) -> TKind,
        FValueMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync;

    fn map_kind<TKind, FKindMapper>(
        self,
        kind_mapper: FKindMapper,
    ) -> MapParser<TInput, Self, TInput, fn(TInput) -> TInput>
    where
        TKind: Into<String>,
        FKindMapper: FnOnce(&str) -> TKind;

    fn map_value<FValueMapper>(
        self,
        value_mapper: FValueMapper,
    ) -> MapParser<TInput, Self, TOutput, FValueMapper>
    where
        FValueMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync;
}

impl<TInput, TParser, TOutput> Map<TInput, TOutput> for TParser
where
    TInput: Send + Sync,
    TParser: Parser<Output = TInput>,
    TOutput: Send + Sync,
{
    fn map<TKind, FKindMapper, FValueMapper>(
        self,
        kind_mapper: FKindMapper,
        value_mapper: FValueMapper,
    ) -> MapParser<TInput, Self, TOutput, FValueMapper>
    where
        TKind: Into<String>,
        FKindMapper: FnOnce(&str) -> TKind,
        FValueMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
    {
        MapParser::new(self, kind_mapper, value_mapper)
    }

    fn map_kind<TKind, FKindMapper>(
        self,
        kind_mapper: FKindMapper,
    ) -> MapParser<TInput, Self, TInput, fn(TInput) -> TInput>
    where
        TKind: Into<String>,
        FKindMapper: FnOnce(&str) -> TKind,
    {
        MapParser::new_kind(self, kind_mapper)
    }

    fn map_value<FValueMapper>(
        self,
        value_mapper: FValueMapper,
    ) -> MapParser<TInput, Self, TOutput, FValueMapper>
    where
        FValueMapper: (Fn(TInput) -> TOutput) + Clone + Send + Sync,
    {
        MapParser::new_value(self, value_mapper)
    }
}
