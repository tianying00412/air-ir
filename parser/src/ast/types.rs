use super::*;

/// The types of values which can be represented in an AirScript program
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    /// A field element
    Felt,
    /// A vector of N integers
    Vector(usize),
    /// A matrix of N rows and M columns
    Matrix(usize, usize),
}
impl Type {
    /// Returns true if this type is an aggregate
    #[inline]
    pub fn is_aggregate(&self) -> bool {
        match self {
            Self::Felt => false,
            Self::Vector(_) | Self::Matrix(_, _) => true,
        }
    }

    /// Returns true if this type is a scalar
    #[inline]
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Felt)
    }

    /// Returns true if this type is a valid iterable in a comprehension
    #[inline]
    pub fn is_iterable(&self) -> bool {
        self.is_vector()
    }

    /// Returns true if this type is a vector
    #[inline]
    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_))
    }

    /// Return a new [Type] representing the type of the value produced by the given [AccessType]
    pub fn access(&self, access_type: AccessType) -> Result<Self, InvalidAccessError> {
        match *self {
            ty if access_type == AccessType::Default => Ok(ty),
            Self::Felt => Err(InvalidAccessError::IndexIntoScalar),
            Self::Vector(len) => match access_type {
                AccessType::Slice(range) if range.end > len => {
                    Err(InvalidAccessError::IndexOutOfBounds)
                }
                AccessType::Slice(range) => Ok(Self::Vector(range.end - range.start)),
                AccessType::Index(idx) if idx >= len => Err(InvalidAccessError::IndexOutOfBounds),
                AccessType::Index(_) => Ok(Self::Felt),
                AccessType::Matrix(_, _) => Err(InvalidAccessError::IndexIntoScalar),
                _ => unreachable!(),
            },
            Self::Matrix(rows, cols) => match access_type {
                AccessType::Slice(range) if range.end > rows => {
                    Err(InvalidAccessError::IndexOutOfBounds)
                }
                AccessType::Slice(range) => Ok(Self::Matrix(range.end - range.start, cols)),
                AccessType::Index(idx) if idx >= rows => Err(InvalidAccessError::IndexOutOfBounds),
                AccessType::Index(_) => Ok(Self::Vector(cols)),
                AccessType::Matrix(row, col) if row >= rows || col >= cols => {
                    Err(InvalidAccessError::IndexOutOfBounds)
                }
                AccessType::Matrix(_, _) => Ok(Self::Felt),
                _ => unreachable!(),
            },
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Felt => f.write_str("field element"),
            Self::Vector(n) => write!(f, "vector of length {}", n),
            Self::Matrix(rows, cols) => write!(f, "matrix of {} rows and {} columns", rows, cols),
        }
    }
}

/// Represents the type signature of a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionType {
    /// An evaluator function, which has no results, and has
    /// a complex type signature due to the nature of trace bindings
    Evaluator(Vec<TraceSegment>),
    /// A standard function with one or more inputs, and a result
    #[allow(dead_code)]
    Function(Vec<Type>, Type),
}
impl FunctionType {
    pub fn result(&self) -> Option<Type> {
        match self {
            Self::Evaluator(_) => None,
            Self::Function(_, result) => Some(*result),
        }
    }
}
