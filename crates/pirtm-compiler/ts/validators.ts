import { z } from 'zod';

// --- Base Types ---
const FixedPointSchema = z.number().int().describe("Scaled by 1,000,000 (SCALE_BASE)");
const PrimeIndexSchema = z.number().int().positive().describe("Unique prime channel identifier");

// --- ADR-004/ADR-088 MLIR Attribute Schema ---
export const PirtmModuleAttributesSchema = z.object({
  prime_index: PrimeIndexSchema,
  epsilon: FixedPointSchema.refine((val) => val > 0, { message: "Positive epsilon required" }),
  op_norm_T: FixedPointSchema.refine((val) => val > 0, { message: "Positive operator norm required" }),
  sigma: FixedPointSchema.refine((val) => val > 0, { message: "Positive sigma required" }),
  alpha: FixedPointSchema.refine((val) => val > 500_000, { message: "α must be > 0.5" }),
  xi_block_dim: z.number().int().min(1).default(1),
  gap_lb: FixedPointSchema,
  slope_ub: FixedPointSchema,
}).refine(
  (data) => (BigInt(data.epsilon) * BigInt(data.op_norm_T)) < 1_000_000_000_000n,
  { message: "Contractivity violation (ε * ‖T‖ < 1)", path: ["epsilon"] }
);

// --- Spectral Certification Schema ---
export const SpectralCertificationSchema = z.object({
  gap_lb: FixedPointSchema.refine((val) => val > 0),
  slope_ub: FixedPointSchema,
  frame_invariant: z.boolean().refine((val) => val === true, { message: "Theorem 13 violation" }),
  proof_artifact_path: z.string().endsWith('.lean'),
});

// --- Transformers ---
export const transformMLIRAttributes = (raw: unknown) => {
  return PirtmModuleAttributesSchema.parse(raw);
};

export const transformSpectralCertification = (raw: unknown) => {
  return SpectralCertificationSchema.parse(raw);
};

export type PirtmModuleAttributes = z.infer<typeof PirtmModuleAttributesSchema>;
export type SpectralCertification = z.infer<typeof SpectralCertificationSchema>;
