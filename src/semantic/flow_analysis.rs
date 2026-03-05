//! Control flow and data flow analysis

use crate::syntax::{SyntaxNode, SyntaxKind};
use crate::diagnostics::{DiagnosticBag, Diagnostic, DiagnosticCode};
use super::symbols::SymbolTable;
use std::collections::{HashMap, HashSet};

/// Flow analyzer for control and data flow analysis
pub struct FlowAnalyzer<'a> {
    /// Symbol table
    symbol_table: &'a SymbolTable,
    /// Diagnostics
    diagnostics: &'a mut DiagnosticBag,
    /// Control flow graph
    cfg: ControlFlowGraph,
    /// Data flow state
    data_flow: DataFlowState,
}

/// Control flow graph
#[derive(Debug)]
struct ControlFlowGraph {
    /// Basic blocks
    blocks: Vec<BasicBlock>,
    /// Entry block index
    entry: usize,
    /// Exit block index
    exit: usize,
}

/// Basic block in the CFG
#[derive(Debug)]
struct BasicBlock {
    /// Block ID
    id: usize,
    /// Statements in the block
    statements: Vec<FlowStatement>,
    /// Predecessors
    predecessors: Vec<usize>,
    /// Successors
    successors: Vec<usize>,
    /// Jump kind
    jump: JumpKind,
}

/// Kind of jump at the end of a basic block
#[derive(Debug, Clone)]
enum JumpKind {
    /// Unconditional jump to successor
    Goto,
    /// Conditional jump based on condition
    Conditional { true_target: usize, false_target: usize },
    /// Return from method
    Return,
    /// Throw exception
    Throw,
}

/// Statement in flow analysis
#[derive(Debug, Clone)]
struct FlowStatement {
    /// Statement kind
    kind: FlowStatementKind,
    /// Source location
    location: SourceLocation,
}

/// Kind of flow statement
#[derive(Debug, Clone)]
enum FlowStatementKind {
    /// Variable assignment
    Assignment { target: String, source: FlowExpression },
    /// Variable declaration
    Declaration { name: String, initializer: Option<FlowExpression> },
    /// Expression evaluation
    Expression(FlowExpression),
    /// Label
    Label(String),
}

/// Expression in flow analysis
#[derive(Debug, Clone)]
enum FlowExpression {
    /// Variable reference
    Variable(String),
    /// Literal value
    Literal,
    /// Binary operation
    Binary { left: Box<FlowExpression>, right: Box<FlowExpression> },
    /// Method call
    Call { method: String, arguments: Vec<FlowExpression> },
    /// Other expression
    Other,
}

/// Source location
#[derive(Debug, Clone, Copy)]
struct SourceLocation {
    // Simplified - would include file, line, column
    offset: usize,
}

/// Data flow analysis state
#[derive(Debug)]
struct DataFlowState {
    /// Definite assignment state at each point
    definite_assignment: HashMap<usize, DefiniteAssignmentState>,
    /// Reachability state at each point
    reachability: HashMap<usize, bool>,
    /// Null state tracking
    null_state: HashMap<usize, NullStateMap>,
}

/// Definite assignment state
#[derive(Debug, Clone, Default)]
struct DefiniteAssignmentState {
    /// Variables definitely assigned at this point
    assigned: HashSet<String>,
    /// Variables definitely unassigned at this point
    unassigned: HashSet<String>,
}

/// Null state for variables
#[derive(Debug, Clone, Default)]
struct NullStateMap {
    /// Variables known to be null
    null_vars: HashSet<String>,
    /// Variables known to be non-null
    non_null_vars: HashSet<String>,
}

impl<'a> FlowAnalyzer<'a> {
    /// Create a new flow analyzer
    pub fn new(symbol_table: &'a SymbolTable, diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            symbol_table,
            diagnostics,
            cfg: ControlFlowGraph::new(),
            data_flow: DataFlowState::new(),
        }
    }
    
    /// Analyze a syntax tree
    pub fn analyze(&mut self, root: &SyntaxNode) {
        // Build control flow graph
        self.build_cfg(root);
        
        // Perform definite assignment analysis
        self.analyze_definite_assignment();
        
        // Perform reachability analysis
        self.analyze_reachability();
        
        // Perform null state analysis
        self.analyze_null_state();
        
        // Check for common issues
        self.check_unreachable_code();
        self.check_unassigned_variables();
        self.check_null_references();
    }
    
    /// Build control flow graph
    fn build_cfg(&mut self, node: &SyntaxNode) {
        // Traverse AST and build CFG
        // This is a simplified implementation
        match node.kind {
            SyntaxKind::MethodDeclaration => self.build_method_cfg(node),
            _ => {
                // Recursively process children
                for child in &node.children {
                    if let crate::syntax::SyntaxElement::Node(child_node) = child {
                        self.build_cfg(child_node);
                    }
                }
            }
        }
    }
    
    /// Build CFG for a method
    fn build_method_cfg(&mut self, _node: &SyntaxNode) {
        // Create entry and exit blocks
        // Process method body
        // Connect blocks based on control flow
    }
    
    /// Analyze definite assignment
    fn analyze_definite_assignment(&mut self) {
        // Compute definite assignment state using data flow analysis
        // Forward analysis to track which variables are definitely assigned
        
        // Initialize entry block
        let mut worklist: Vec<usize> = vec![self.cfg.entry];
        let mut visited = HashSet::new();
        
        while let Some(block_id) = worklist.pop() {
            if !visited.insert(block_id) {
                continue;
            }
            
            // Process statements in block
            let mut state = self.get_predecessor_state(block_id);
            let mut successors = Vec::new();
            
            {
                let block = &self.cfg.blocks[block_id];
                
                for statement in &block.statements {
                    match &statement.kind {
                        FlowStatementKind::Assignment { target, .. } => {
                            state.assigned.insert(target.clone());
                            state.unassigned.remove(target);
                        }
                        FlowStatementKind::Declaration { name, initializer } => {
                            if initializer.is_some() {
                                state.assigned.insert(name.clone());
                            } else {
                                state.unassigned.insert(name.clone());
                            }
                        }
                        _ => {}
                    }
                }
                
                successors = block.successors.clone();
            }
            
            // Update state and add successors to worklist
            self.update_definite_assignment_state(block_id, state);
            worklist.extend(successors);
        }
    }
    
    /// Analyze reachability
    fn analyze_reachability(&mut self) {
        // Mark reachable blocks starting from entry
        let mut reachable = HashSet::new();
        let mut worklist = vec![self.cfg.entry];
        
        while let Some(block_id) = worklist.pop() {
            if !reachable.insert(block_id) {
                continue;
            }
            
            let block = &self.cfg.blocks[block_id];
            
            // Add successors based on jump kind
            match &block.jump {
                JumpKind::Goto => {
                    worklist.extend(&block.successors);
                }
                JumpKind::Conditional { true_target, false_target } => {
                    // Both branches are potentially reachable
                    worklist.push(*true_target);
                    worklist.push(*false_target);
                }
                JumpKind::Return | JumpKind::Throw => {
                    // No successors
                }
            }
        }
        
        // Mark unreachable blocks
        let block_count = self.cfg.blocks.len();
        for id in 0..block_count {
            self.update_reachability(id, reachable.contains(&id));
        }
    }
    
    /// Analyze null state
    fn analyze_null_state(&mut self) {
        // Track null/non-null state of reference variables
        // This helps with null reference warnings
        
        let mut worklist: Vec<usize> = vec![self.cfg.entry];
        let mut visited = HashSet::new();
        
        while let Some(block_id) = worklist.pop() {
            if !visited.insert(block_id) {
                continue;
            }
            
            let mut state = self.get_predecessor_null_state(block_id);
            let mut successors = Vec::new();
            
            {
                let block = &self.cfg.blocks[block_id];
                
                for statement in &block.statements {
                    match &statement.kind {
                        FlowStatementKind::Assignment { target, source } => {
                            // Update null state based on assignment
                            match source {
                                FlowExpression::Literal => {
                                    // Could be null literal
                                    state.null_vars.insert(target.clone());
                                    state.non_null_vars.remove(target);
                                }
                                _ => {
                                    // Conservative: assume non-null
                                    state.non_null_vars.insert(target.clone());
                                    state.null_vars.remove(target);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                
                successors = block.successors.clone();
            }
            
            self.update_null_state(block_id, state);
            worklist.extend(successors);
        }
    }
    
    /// Check for unreachable code
    fn check_unreachable_code(&mut self) {
        let block_count = self.cfg.blocks.len();
        for id in 0..block_count {
            if !self.is_reachable(id) && id != self.cfg.exit {
                // Report unreachable code
                let statement_count = self.cfg.blocks[id].statements.len();
                for _ in 0..statement_count {
                    self.diagnostics.add(Diagnostic::warning(
                        DiagnosticCode::UnreachableCode,
                        "Unreachable code detected".to_string(),
                    ));
                }
            }
        }
    }
    
    /// Check for unassigned variables
    fn check_unassigned_variables(&mut self) {
        // Check variable uses for definite assignment
        let block_count = self.cfg.blocks.len();
        for block_id in 0..block_count {
            let state = self.get_definite_assignment_state(block_id);
            let statements = self.cfg.blocks[block_id].statements.clone();
            
            for statement in &statements {
                self.check_statement_assignment(statement, &state);
            }
        }
    }
    
    /// Check for potential null references
    fn check_null_references(&mut self) {
        let block_count = self.cfg.blocks.len();
        for block_id in 0..block_count {
            let state = self.get_null_state(block_id);
            let statements = self.cfg.blocks[block_id].statements.clone();
            
            for statement in &statements {
                self.check_statement_null_safety(statement, &state);
            }
        }
    }
    
    // Helper methods
    
    fn get_predecessor_state(&self, block_id: usize) -> DefiniteAssignmentState {
        let block = &self.cfg.blocks[block_id];
        
        if block.predecessors.is_empty() {
            // Entry block or unreachable
            DefiniteAssignmentState {
                assigned: HashSet::new(),
                unassigned: HashSet::new(),
            }
        } else {
            // Merge predecessor states
            let mut result = DefiniteAssignmentState {
                assigned: HashSet::new(),
                unassigned: HashSet::new(),
            };
            
            // A variable is definitely assigned if assigned in all predecessors
            // A variable is definitely unassigned if unassigned in all predecessors
            
            for (i, &pred_id) in block.predecessors.iter().enumerate() {
                if let Some(pred_state) = self.data_flow.definite_assignment.get(&pred_id) {
                    if i == 0 {
                        result.assigned = pred_state.assigned.clone();
                        result.unassigned = pred_state.unassigned.clone();
                    } else {
                        // Intersection for assigned, union for unassigned
                        result.assigned = result.assigned.intersection(&pred_state.assigned)
                            .cloned().collect();
                        result.unassigned = result.unassigned.union(&pred_state.unassigned)
                            .cloned().collect();
                    }
                }
            }
            
            result
        }
    }
    
    fn get_predecessor_null_state(&self, block_id: usize) -> NullStateMap {
        // Similar to get_predecessor_state but for null analysis
        NullStateMap {
            null_vars: HashSet::new(),
            non_null_vars: HashSet::new(),
        }
    }
    
    fn update_definite_assignment_state(&mut self, block_id: usize, state: DefiniteAssignmentState) {
        self.data_flow.definite_assignment.insert(block_id, state);
    }
    
    fn update_reachability(&mut self, block_id: usize, reachable: bool) {
        self.data_flow.reachability.insert(block_id, reachable);
    }
    
    fn update_null_state(&mut self, block_id: usize, state: NullStateMap) {
        self.data_flow.null_state.insert(block_id, state);
    }
    
    fn get_definite_assignment_state(&self, block_id: usize) -> DefiniteAssignmentState {
        self.data_flow.definite_assignment.get(&block_id)
            .cloned()
            .unwrap_or_default()
    }
    
    fn get_null_state(&self, block_id: usize) -> NullStateMap {
        self.data_flow.null_state.get(&block_id)
            .cloned()
            .unwrap_or_default()
    }
    
    fn is_reachable(&self, block_id: usize) -> bool {
        self.data_flow.reachability.get(&block_id).copied().unwrap_or(false)
    }
    
    fn check_statement_assignment(&mut self, statement: &FlowStatement, state: &DefiniteAssignmentState) {
        // Check if variables used in statement are definitely assigned
        match &statement.kind {
            FlowStatementKind::Expression(expr) => {
                self.check_expr_assignment(expr, state);
            }
            FlowStatementKind::Assignment { source, .. } => {
                self.check_expr_assignment(source, state);
            }
            _ => {}
        }
    }
    
    fn check_expr_assignment(&mut self, expr: &FlowExpression, state: &DefiniteAssignmentState) {
        match expr {
            FlowExpression::Variable(name) => {
                if !state.assigned.contains(name) {
                    self.diagnostics.add(Diagnostic::error(
                        DiagnosticCode::UseOfUnassignedVariable,
                        format!("Use of unassigned variable '{}'", name),
                    ));
                }
            }
            FlowExpression::Binary { left, right } => {
                self.check_expr_assignment(left, state);
                self.check_expr_assignment(right, state);
            }
            FlowExpression::Call { arguments, .. } => {
                for arg in arguments {
                    self.check_expr_assignment(arg, state);
                }
            }
            _ => {}
        }
    }
    
    fn check_statement_null_safety(&mut self, statement: &FlowStatement, state: &NullStateMap) {
        // Check for potential null dereferences
        match &statement.kind {
            FlowStatementKind::Expression(expr) => {
                self.check_expr_null_safety(expr, state);
            }
            _ => {}
        }
    }
    
    fn check_expr_null_safety(&mut self, expr: &FlowExpression, state: &NullStateMap) {
        match expr {
            FlowExpression::Call { method, .. } => {
                // Check if we're calling a method on a null reference
                if state.null_vars.contains(method) {
                    self.diagnostics.add(Diagnostic::warning(
                        DiagnosticCode::PossibleNullReference,
                        "Possible null reference exception".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }
}

impl ControlFlowGraph {
    fn new() -> Self {
        // Create initial CFG with entry and exit blocks
        let entry = BasicBlock {
            id: 0,
            statements: Vec::new(),
            predecessors: Vec::new(),
            successors: vec![1],
            jump: JumpKind::Goto,
        };
        
        let exit = BasicBlock {
            id: 1,
            statements: Vec::new(),
            predecessors: vec![0],
            successors: Vec::new(),
            jump: JumpKind::Return,
        };
        
        Self {
            blocks: vec![entry, exit],
            entry: 0,
            exit: 1,
        }
    }
}

impl DataFlowState {
    fn new() -> Self {
        Self {
            definite_assignment: HashMap::new(),
            reachability: HashMap::new(),
            null_state: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cfg_creation() {
        let cfg = ControlFlowGraph::new();
        assert_eq!(cfg.blocks.len(), 2);
        assert_eq!(cfg.entry, 0);
        assert_eq!(cfg.exit, 1);
    }
    
    #[test]
    fn test_definite_assignment_state() {
        let mut state = DefiniteAssignmentState {
            assigned: HashSet::new(),
            unassigned: HashSet::new(),
        };
        
        state.assigned.insert("x".to_string());
        state.unassigned.insert("y".to_string());
        
        assert!(state.assigned.contains("x"));
        assert!(!state.assigned.contains("y"));
        assert!(state.unassigned.contains("y"));
        assert!(!state.unassigned.contains("x"));
    }
}