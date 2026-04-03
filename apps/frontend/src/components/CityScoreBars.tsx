import { type CityScore } from '@/lib/api';

interface Props {
    score: CityScore;
}

const MAX_SCORE = 2000;

function ScoreBar({ label, value, color }: { label: string; value: number; color: string }) {
    const percentage = Math.min(100, Math.max(0, (value / MAX_SCORE) * 100));

    return (
        <div className="score-group">
            <div className="score-label">
                <span>{label}</span>
                <span style={{ color: value > 1500 ? 'var(--success)' : value < 500 ? 'var(--danger)' : 'var(--text-main)' }}>
                    {value}
                </span>
            </div>
            <div className="score-bar-bg">
                <div
                    className="score-bar-fill"
                    style={{ width: `${percentage}%`, backgroundColor: color }}
                />
            </div>
        </div>
    );
}

export default function CityScoreBars({ score }: Props) {
    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            <ScoreBar label="Quality of Life" value={score.quality_of_life} color="#10b981" />
            <ScoreBar label="Safety" value={score.safety} color="#3b82f6" />
            <ScoreBar label="Economy" value={score.economy} color="#f59e0b" />
            <ScoreBar label="Culture" value={score.culture} color="#ec4899" />
        </div>
    );
}
