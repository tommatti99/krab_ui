#[allow(dead_code)]
struct Color {
    pct_05: String,  
    pct_10: String,  
    pct_20: String,  
    pct_30: String,  
    pct_40: String,  
    pct_50: String,  
    pct_60: String,  
    pct_70: String,  
    pct_80: String,  
    pct_90: String,  
    pct_95: String,  
}

#[allow(dead_code)]
impl Color {
    pub fn black() -> Self {
        Self { pct_05: "#fafafa".to_string(), pct_10: "#f4f4f5".to_string(), pct_20: "#e4e4e7".to_string(), pct_30: "#d4d4d8".to_string(), pct_40: "#a1a1aa".to_string(), pct_50: "#71717a".to_string(), 
               pct_60: "#52525b".to_string(), pct_70: "#3f3f46".to_string(), pct_80: "#27272a".to_string(), pct_90: "#18181b".to_string(), pct_95: "#09090b".to_string() }
    }

    pub fn red() -> Self {
        Self { pct_05: "#fef2f2".to_string(), pct_10: "#fee2e2".to_string(), pct_20: "#fecaca".to_string(), pct_30: "#fca5a5".to_string(), pct_40: "#f87171".to_string(), pct_50: "#ef4444".to_string(), 
               pct_60: "#dc2626".to_string(), pct_70: "#b91c1c".to_string(), pct_80: "#991b1b".to_string(), pct_90: "#7f1d1d".to_string(), pct_95: "#450a0a".to_string() }
    }
    
    pub fn orange() -> Self {
        Self { pct_05: "#fff7ed".to_string(), pct_10: "#ffedd5".to_string(), pct_20: "#fed7aa".to_string(), pct_30: "#fdba74".to_string(), pct_40: "#fb923c".to_string(), pct_50: "#f97316".to_string(), 
               pct_60: "#ea580c".to_string(), pct_70: "#c2410c".to_string(), pct_80: "#9a3412".to_string(), pct_90: "#7c2d12".to_string(), pct_95: "#431407".to_string() }
    }

    pub fn gold() -> Self {
        Self { pct_05: "#fffbeb".to_string(), pct_10: "#fef3c7".to_string(), pct_20: "#fde68a".to_string(), pct_30: "#fcd34d".to_string(), pct_40: "#fbbf24".to_string(), pct_50: "#f59e0b".to_string(), 
               pct_60: "#d97706".to_string(), pct_70: "#b45309".to_string(), pct_80: "#92400e".to_string(), pct_90: "#78350f".to_string(), pct_95: "#451a03".to_string() }
    }

    pub fn yellow() -> Self {
        Self { pct_05: "#fefce8".to_string(), pct_10: "#fef9c3".to_string(), pct_20: "#fef08a".to_string(), pct_30: "#fde047".to_string(), pct_40: "#facc15".to_string(), pct_50: "#eab308".to_string(), 
               pct_60: "#ca8a04".to_string(), pct_70: "#a16207".to_string(), pct_80: "#854d0e".to_string(), pct_90: "#713f12".to_string(), pct_95: "#422006".to_string() }
    }

    pub fn midnight_blue() -> Self {
        Self { pct_05: "#f8fafc".to_string(), pct_10: "#f1f5f9".to_string(), pct_20: "#e2e8f0".to_string(), pct_30: "#cbd5e1".to_string(), pct_40: "#94a3b8".to_string(), pct_50: "#64748b".to_string(), 
               pct_60: "#475569".to_string(), pct_70: "#334155".to_string(), pct_80: "#1e293b".to_string(), pct_90: "#0f172a".to_string(), pct_95: "#020617".to_string() }
    }

    pub fn cobalt() -> Self {
        Self { pct_05: "#f0f9ff".to_string(), pct_10: "#e0f2fe".to_string(), pct_20: "#bae6fd".to_string(), pct_30: "#7dd3fc".to_string(), pct_40: "#38bdf8".to_string(), pct_50: "#0ea5e9'".to_string(), 
               pct_60: "#0284c7".to_string(), pct_70: "#0369a1".to_string(), pct_80: "#075985".to_string(), pct_90: "#0c4a6e".to_string(), pct_95: "#082f49".to_string() }
    }

    pub fn blue() -> Self {
        Self { pct_05: "#eff6ff".to_string(), pct_10: "#dbeafe".to_string(), pct_20: "#bfdbfe".to_string(), pct_30: "#93c5fd".to_string(), pct_40: "#60a5fa".to_string(), pct_50: "#3b82f6".to_string(), 
               pct_60: "#2563eb".to_string(), pct_70: "#1d4ed8".to_string(), pct_80: "#1e40af".to_string(), pct_90: "#1e3a8a".to_string(), pct_95: "#172554".to_string() }
    }

    pub fn cyan() -> Self {
        Self { pct_05: "#f0fdfa".to_string(), pct_10: "#ccfbf1".to_string(), pct_20: "#99f6e4".to_string(), pct_30: "#5eead4".to_string(), pct_40: "#2dd4bf".to_string(), pct_50: "#14b8a6".to_string(), 
               pct_60: "#0d9488".to_string(), pct_70: "#0f766e".to_string(), pct_80: "#115e59".to_string(), pct_90: "#134e4a".to_string(), pct_95: "#042f2e".to_string() }
    }

    pub fn green() -> Self {
        Self { pct_05: "#f0fdf4".to_string(), pct_10: "#dcfce7".to_string(), pct_20: "#bbf7d0".to_string(), pct_30: "#86efac".to_string(), pct_40: "#4ade80".to_string(), pct_50: "#22c55e".to_string(), 
               pct_60: "#16a34a".to_string(), pct_70: "#15803d".to_string(), pct_80: "#166534".to_string(), pct_90: "#14532d".to_string(), pct_95: "#052e16".to_string() }
    }

    pub fn lime() -> Self {
        Self { pct_05: "#f7fee7".to_string(), pct_10: "#ecfccb".to_string(), pct_20: "#d9f99d".to_string(), pct_30: "#bef264".to_string(), pct_40: "#a3e635".to_string(), pct_50: "#84cc16".to_string(), 
               pct_60: "#65a30d".to_string(), pct_70: "#4d7c0f".to_string(), pct_80: "#3f6212".to_string(), pct_90: "#365314".to_string(), pct_95: "#1a2e05".to_string() }
    }

    pub fn violet() -> Self {
        Self { pct_05: "#f5f3ff".to_string(), pct_10: "#ede9fe".to_string(), pct_20: "#ddd6fe".to_string(), pct_30: "#c4b5fd".to_string(), pct_40: "#a78bfa".to_string(), pct_50: "#8b5cf6".to_string(), 
               pct_60: "#7c3aed".to_string(), pct_70: "#6d28d9".to_string(), pct_80: "#5b21b6".to_string(), pct_90: "#4c1d95".to_string(), pct_95: "#1e1b4b".to_string() }
    }

    pub fn purple() -> Self {
        Self { pct_05: "#faf5ff".to_string(), pct_10: "#f3e8ff".to_string(), pct_20: "#e9d5ff".to_string(), pct_30: "#d8b4fe".to_string(), pct_40: "#c084fc".to_string(), pct_50: "#a855f7".to_string(), 
               pct_60: "#9333ea".to_string(), pct_70: "#7e22ce".to_string(), pct_80: "#6b21a8".to_string(), pct_90: "#581c87".to_string(), pct_95: "#3b0764".to_string() }
    }

    pub fn magenta() -> Self {
        Self { pct_05: "#fdf4ff".to_string(), pct_10: "#fae8ff".to_string(), pct_20: "#f5d0fe".to_string(), pct_30: "#f0abfc".to_string(), pct_40: "#e879f9".to_string(), pct_50: "#d946ef".to_string(), 
               pct_60: "#c026d3".to_string(), pct_70: "#a21caf".to_string(), pct_80: "#86198f".to_string(), pct_90: "#701a75".to_string(), pct_95: "#4a044e".to_string() }
    }

    pub fn pink() -> Self {
        Self { pct_05: "#fdf2f8".to_string(), pct_10: "#fce7f3".to_string(), pct_20: "#fbcfe8".to_string(), pct_30: "#f9a8d4".to_string(), pct_40: "#f472b6".to_string(), pct_50: "#ec4899".to_string(), 
               pct_60: "#db2777".to_string(), pct_70: "#be185d".to_string(), pct_80: "#9d174d".to_string(), pct_90: "#831843".to_string(), pct_95: "#500724".to_string() }
    }

    pub fn rose() -> Self {
        Self { pct_05: "#fff1f2".to_string(), pct_10: "#ffe4e6".to_string(), pct_20: "#fecdd3".to_string(), pct_30: "#fda4af".to_string(), pct_40: "#fb7185".to_string(), pct_50: "#f43f5e".to_string(), 
               pct_60: "#e11d48".to_string(), pct_70: "#be123c".to_string(), pct_80: "#9f1239".to_string(), pct_90: "#881337".to_string(), pct_95: "#4c0519".to_string() }
    }
}