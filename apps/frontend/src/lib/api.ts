export interface Offer {
    id: string;
    title: string;
    link: string;
    city: string;
    domain: string;
    salary: number;
    start_date: string;
    end_date: string;
    available: boolean;
}

export interface CityScore {
    city: string;
    country: string;
    updated_at: string;
    quality_of_life: number;
    safety: number;
    economy: number;
    culture: number;
}

export interface News {
    id: string;
    name: string;
    source: string;
    date: string;
    tags: string[];
    city: string;
    country: string;
}

export interface AggregatedOffer {
    offer: Offer;
    city_score: CityScore | null;
    latest_news: News[] | null;
}

export interface CreateStudentRequest {
    firstname: string;
    name: string;
    domain: string;
}

export interface Student {
    id: string;
    firstname: string;
    name: string;
    domain: string;
}

export interface Internship {
    id: string;
    student_id: string;
    offer_id: string;
    status: 'Approved' | 'Rejected';
    message: string;
}

const API_BASE = 'http://localhost:8080';

export async function fetchOffers(params?: { domain?: string; city?: string; limit?: number }): Promise<AggregatedOffer[]> {
    const searchParams = new URLSearchParams();
    if (params?.domain) searchParams.append('domain', params.domain);
    if (params?.city) searchParams.append('city', params.city);
    if (params?.limit) searchParams.append('limit', String(params.limit));

    const qs = searchParams.toString();
    const res = await fetch(`${API_BASE}/offer${qs ? '?' + qs : ''}`);
    if (!res.ok) throw new Error('Failed to fetch offers');
    return res.json();
}

export async function fetchStudent(id: string): Promise<Student> {
    const res = await fetch(`${API_BASE}/student/${id}`);
    if (!res.ok) throw new Error('Student not found');
    return res.json();
}

export async function fetchStudentsByDomain(domain: string): Promise<Student[]> {
    const res = await fetch(`${API_BASE}/student?domain=${encodeURIComponent(domain)}`);
    if (!res.ok) throw new Error('Failed to fetch students');
    return res.json();
}

export async function createStudent(data: CreateStudentRequest): Promise<Student> {
    const res = await fetch(`${API_BASE}/student`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
    });

    const result = await res.json();
    if (!res.ok) throw new Error(result.error || 'Failed to create student');
    return result;
}

export async function fetchRecommendedOffers(studentId: string): Promise<AggregatedOffer[]> {
    const res = await fetch(`${API_BASE}/student/${studentId}/recommended-offers`);
    if (!res.ok) throw new Error('Failed to fetch recommended offers');
    return res.json();
}

export async function applyForInternship(studentId: string, offerId: string): Promise<Internship> {
    const res = await fetch(`${API_BASE}/internship`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ student_id: studentId, offer_id: offerId })
    });

    const data = await res.json();
    if (!res.ok) throw new Error(data.error || 'Application failed');
    return data;
}

export type SortByScore = 'quality_of_life' | 'safety' | 'economy' | 'culture';

export function sortOffersByScore(offers: AggregatedOffer[], sortBy: SortByScore): AggregatedOffer[] {
    return [...offers].sort((a, b) => {
        const scoreA = a.city_score?.[sortBy] ?? 0;
        const scoreB = b.city_score?.[sortBy] ?? 0;
        return scoreB - scoreA;
    });
}

// --- Notifications (Polytech) ---

export interface Notification {
    id: string;
    studentId: string;
    type: string;
    offerId: string;
    message: string;
    read: boolean;
}

export async function fetchNotifications(studentId: string): Promise<Notification[]> {
    const res = await fetch(`${API_BASE}/students/${studentId}/notifications`);
    if (!res.ok) throw new Error('Failed to fetch notifications');
    return res.json();
}

export async function markNotificationRead(notificationId: string): Promise<void> {
    const res = await fetch(`${API_BASE}/notifications/${notificationId}/read`, { method: 'PUT' });
    if (!res.ok) throw new Error('Failed to mark notification as read');
}

// --- Subscriber Preferences (La Poste) ---

const LA_POSTE_BASE = 'http://localhost:8083';

export interface Subscriber {
    studentId: string;
    domain: string;
    channel: 'email' | 'discord';
    contact: string;
    enabled: boolean;
}

export async function fetchSubscriber(studentId: string): Promise<Subscriber> {
    const res = await fetch(`${LA_POSTE_BASE}/subscribers/${studentId}`);
    if (!res.ok) throw new Error('Subscriber not found');
    return res.json();
}

export async function updateSubscriber(studentId: string, data: Partial<Omit<Subscriber, 'studentId' | 'domain'>>): Promise<Subscriber> {
    const res = await fetch(`${LA_POSTE_BASE}/subscribers/${studentId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error('Failed to update preferences');
    return res.json();
}

export async function deleteSubscriber(studentId: string): Promise<void> {
    const res = await fetch(`${LA_POSTE_BASE}/subscribers/${studentId}`, { method: 'DELETE' });
    if (!res.ok) throw new Error('Failed to unsubscribe');
}
