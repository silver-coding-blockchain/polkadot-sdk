// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Collection of behavior variants.

mod back_garbage_candidate;
mod common;
mod dispute_finalized_candidates;
mod dispute_valid_candidates;
mod spam_statement_requests;
mod suggest_garbage_candidate;
mod support_disabled;

pub(crate) use self::{
	back_garbage_candidate::{BackGarbageCandidateOptions, BackGarbageCandidates},
	dispute_finalized_candidates::{DisputeFinalizedCandidates, DisputeFinalizedCandidatesOptions},
	dispute_valid_candidates::{DisputeAncestorOptions, DisputeValidCandidates},
	spam_statement_requests::{SpamStatementRequests, SpamStatementRequestsOptions},
	suggest_garbage_candidate::{SuggestGarbageCandidateOptions, SuggestGarbageCandidates},
	support_disabled::{SupportDisabled, SupportDisabledOptions},
};
pub(crate) use common::*;
