# Lit - AI Speech-to-Text Desktop Application

## Concept & Vision

**Lit** is a premium, privacy-first speech-to-text desktop application that transforms voice into text with a single keystroke. The experience should feel like having a professional transcriptionist built into your computer—one that never sleeps, never judges, and never uploads your voice to the cloud.

The design language embodies **quiet confidence**: sophisticated enough for power users, approachable enough for anyone. Think of it as the intersection of a high-end audio workstation and a minimalist productivity tool—functional precision wrapped in refined aesthetics.

---

## Design Language

### Aesthetic Direction
**Reference**: Linear meets Raycast—dark-mode-first with glass morphism accents, subtle depth through layered surfaces, and micro-animations that feel responsive without being distracting. The UI should feel like a precision instrument: every element purposeful, nothing decorative for its own sake.

### Color Palette
```css
:root {
  /* Backgrounds - Layered depth system */
  --bg-base: #0a0a0b;           /* Deepest layer */
  --bg-surface: #111113;         /* Card/panel backgrounds */
  --bg-elevated: #18181b;        /* Modals, popovers, hover states */
  --bg-overlay: rgba(24, 24, 27, 0.85); /* Glass overlays */
  
  /* Borders */
  --border-subtle: rgba(255, 255, 255, 0.06);
  --border-default: rgba(255, 255, 255, 0.1);
  --border-focus: rgba(139, 92, 246, 0.5);
  
  /* Text */
  --text-primary: #fafafa;
  --text-secondary: #a1a1aa;
  --text-tertiary: #52525b;
  --text-inverse: #0a0a0b;
  
  /* Accent - Electric Violet */
  --accent-primary: #8b5cf6;
  --accent-primary-hover: #a78bfa;
  --accent-primary-muted: rgba(139, 92, 246, 0.15);
  
  /* Semantic */
  --success: #22c55e;
  --success-muted: rgba(34, 197, 94, 0.15);
  --warning: #f59e0b;
  --warning-muted: rgba(245, 158, 11, 0.15);
  --error: #ef4444;
  --error-muted: rgba(239, 68, 68, 0.15);
  --info: #3b82f6;
  --info-muted: rgba(59, 130, 246, 0.15);
  
  /* Recording state - Pulsing coral */
  --recording: #f43f5e;
  --recording-glow: rgba(244, 63, 94, 0.4);
}
```

### Typography
```css
/* Primary: Inter - Clean, professional, excellent readability */
--font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;

/* Monospace: JetBrains Mono - For shortcuts, code, technical values */
--font-mono: 'JetBrains Mono', 'SF Mono', 'Fira Code', monospace;

/* Scale */
--text-xs: 0.75rem;    /* 12px - Labels, badges */
--text-sm: 0.8125rem;   /* 13px - Secondary text, descriptions */
--text-base: 0.875rem;  /* 14px - Body text */
--text-lg: 1rem;        /* 16px - Section headers */
--text-xl: 1.25rem;     /* 20px - Page titles */
--text-2xl: 1.5rem;     /* 24px - Hero elements */

/* Weights */
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
```

### Spatial System
```css
/* 4px base unit */
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;       /* 16px */
--space-5: 1.25rem;   /* 20px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
--space-10: 2.5rem;   /* 40px */
--space-12: 3rem;     /* 48px */

/* Radius */
--radius-sm: 6px;
--radius-md: 8px;
--radius-lg: 12px;
--radius-xl: 16px;
--radius-full: 9999px;
```

### Motion Philosophy
- **Micro-interactions**: 150ms ease-out for hovers, toggles, button presses
- **Panel transitions**: 200ms ease-in-out for expanding/collapsing
- **Page transitions**: 250ms ease-out for section changes
- **Recording pulse**: Continuous 2s ease-in-out for the recording indicator
- **Spring physics**: For draggable elements and gesture-based interactions
- **Stagger animations**: 50ms delay between list items on load

### Visual Assets
- **Icons**: Lucide React - consistent 1.5px stroke weight, 24px default size
- **Decorative**: Subtle gradient orbs for visual interest in empty states
- **Recording indicator**: Animated waveform visualization during active recording

---

## Layout & Structure

### Window Architecture
- **Main Window**: 900px × 650px default, minimum 800px × 550px, resizable
- **Frameless window** with custom title bar (drag region, window controls)
- **Overlay**: Semi-transparent recording indicator (can position at top or bottom of screen)

### Main Layout Grid
```
┌─────────────────────────────────────────────────────────────┐
│  [Custom Title Bar - drag region + window controls]         │
├──────────────┬────────────────────────────────────────────────┤
│              │                                                │
│   Sidebar    │              Content Area                      │
│   (200px)    │              (flex-1)                          │
│              │                                                │
│  - Logo      │   ┌─────────────────────────────────────┐     │
│  - Nav       │   │  Section Header                      │     │
│    items     │   ├─────────────────────────────────────┤     │
│              │   │                                     │     │
│              │   │  Settings Groups                    │     │
│              │   │  (scrollable)                       │     │
│              │   │                                     │     │
│              │   │                                     │     │
│              │   └─────────────────────────────────────┘     │
│              │                                                │
├──────────────┴────────────────────────────────────────────────┤
│  [Footer - Status bar, model state, update info]              │
└─────────────────────────────────────────────────────────────┘
```

### Navigation Sections
1. **General** - Primary shortcuts, language, push-to-talk
2. **Models** - Model selection, download management
3. **Advanced** - Output, transcription, app behavior settings
4. **History** - Past transcriptions, saved items
5. **Post Process** - LLM integration for text refinement (conditional)
6. **About** - Version, links, acknowledgments

### Responsive Strategy
- Sidebar collapses to icon-only at < 900px width
- Settings groups stack vertically on narrow widths
- Touch-friendly tap targets on all interactive elements (min 44px)

---

## Features & Interactions

### Core Recording Flow
1. User presses configured global shortcut
2. Recording overlay appears with pulsing animation
3. Real-time audio waveform visualization
4. User releases key or presses again to stop
5. Processing state shown with spinner
6. Text appears in target application via paste/typing
7. Toast notification confirms success

### Settings Behaviors

#### Toggles
- **Default**: Off position
- **Hover**: Background lightens to --bg-elevated
- **Active (On)**: Filled with --accent-primary, thumb slides right
- **Active (Off)**: Gray track, thumb on left
- **Disabled**: 50% opacity, cursor not-allowed
- **Loading**: Spinner replaces thumb during async operations

#### Dropdowns/Selects
- **Closed**: Border --border-default, chevron-down icon
- **Open**: Border --border-focus, elevated shadow, chevron-up
- **Option Hover**: Background --bg-elevated
- **Selected Option**: Checkmark icon, accent text
- **Search**: Optional filter input at top of dropdown

#### Sliders
- **Track**: 4px height, --bg-elevated background
- **Fill**: --accent-primary gradient
- **Thumb**: 16px circle, white with shadow
- **Hover**: Thumb scales to 18px
- **Dragging**: Thumb has glow effect
- **Value tooltip**: Appears above thumb during drag

#### Shortcut Inputs
- **Default**: Dashed border, "Press keys..." placeholder
- **Recording**: Border pulses with --accent-primary
- **Recorded**: Solid border, shortcut displayed in monospace (e.g., ⌘⇧S)
- **Conflict**: Red border, error message below
- **Clear**: X button on hover

### Onboarding Flow
1. **Welcome Screen**: App logo, tagline "Speak. Transcribe. Done.", Get Started button
2. **Permissions Screen**: Microphone + Accessibility with status indicators
3. **Model Selection**: Grid of model cards with download progress
4. **Shortcuts Setup**: Optional quick configuration
5. **Complete**: Success state, launch into main app

### Error States
- **Network errors**: Retry button with error message
- **Permission denied**: Link to system settings with explanation
- **Model load failure**: Alternative model suggestions
- **Recording failure**: Clear error message with troubleshooting steps

### Empty States
- **No history**: Illustration + "Your transcriptions will appear here"
- **No models**: Download prompt with recommendations
- **No shortcuts configured**: Setup wizard link

---

## Component Inventory

### Title Bar
- **Default**: Logo left, window controls right (minimize, maximize, close)
- **Hover on controls**: Background highlight
- **Active window**: Full opacity
- **Inactive window**: 80% opacity

### Sidebar Navigation
- **Nav Item Default**: Icon + label, --text-secondary
- **Nav Item Hover**: Background --bg-elevated, --text-primary
- **Nav Item Active**: Background --accent-primary-muted, accent icon, --text-primary
- **Section Badge**: Optional count badge (e.g., update available)

### Settings Group
- **Container**: Rounded-lg, border-subtle, padding --space-4
- **Header**: Section title (--text-lg, font-medium), optional collapse toggle
- **Description**: --text-sm, --text-secondary, max 2 lines

### Setting Row
- **Layout**: Label/description left, control right
- **Label**: --text-base, font-medium
- **Description**: --text-sm, --text-secondary
- **Control area**: Fixed width for aligned controls

### Button Variants
- **Primary**: bg --accent-primary, text white, hover --accent-primary-hover
- **Secondary**: bg --bg-elevated, border --border-default, hover border --border-focus
- **Ghost**: Transparent, hover bg --bg-elevated
- **Danger**: bg --error-muted, text --error, hover bg --error with white text
- **Sizes**: sm (28px), md (36px), lg (44px)

### Input Fields
- **Default**: bg --bg-surface, border --border-default, rounded-md
- **Focus**: border --border-focus, subtle glow
- **Error**: border --error, error message below
- **Disabled**: opacity 50%

### Toast Notifications
- **Success**: Left border --success, icon checkmark
- **Error**: Left border --error, icon X
- **Info**: Left border --info, icon info
- **Position**: Top-right stack, auto-dismiss after 4s

### Recording Overlay
- **Container**: Fixed position, semi-transparent bg --bg-overlay, backdrop-blur
- **Content**: Centered waveform animation, "Listening..." text
- **States**: Idle (hidden), Recording (pulse + waveform), Processing (spinner)

### Model Card
- **Default**: bg --bg-surface, border --border-subtle, rounded-lg
- **Content**: Model name, description, size, capabilities badges
- **Actions**: Download button, delete button (if downloaded)
- **Downloading**: Progress bar overlay, cancel button
- **Active**: Accent border, "Active" badge

### Footer Status Bar
- **Layout**: Left (current model), Center (status message), Right (update indicator)
- **Height**: 36px
- **Background**: --bg-surface with top border

---

## Technical Considerations

### Framework
- **React 18+** with TypeScript (strict mode)
- **Tailwind CSS** for styling
- **Zustand** for state management
- **React-i18next** for internationalization
- **Tauri 2.x** for desktop integration

### State Management
```typescript
interface Settings {
  // General
  shortcuts: {
    transcribe: string;
    cancel: string;
    transcribeWithPostProcess?: string;
  };
  language: string;
  pushToTalk: boolean;
  
  // Models
  activeModel: string;
  downloadedModels: string[];
  
  // Advanced
  microphone: string;
  outputDevice: string;
  acceleration: 'auto' | 'gpu' | 'cpu';
  overlay: 'top' | 'bottom' | 'none';
  pasteMethod: 'clipboard' | 'direct' | 'none';
  // ... many more
  
  // UI State
  debugMode: boolean;
}
```

### Event System
- Backend → Frontend: Tauri events for recording state, model loading, errors
- Frontend → Backend: Commands for settings updates, transcription triggers

### Performance Targets
- Initial load: < 500ms
- Settings change response: < 100ms
- Recording start latency: < 50ms
- Animation frame rate: 60fps

---

## Accessibility

- Full keyboard navigation (Tab, Arrow keys, Enter, Escape)
- ARIA labels on all interactive elements
- Focus visible indicators (ring --accent-primary)
- Screen reader announcements for state changes
- Reduced motion support via prefers-reduced-motion
- Minimum contrast ratio 4.5:1 for text

---

## File Structure (Target)
```
src/
├── components/
│   ├── ui/                    # Primitive components
│   │   ├── Button.tsx
│   │   ├── Input.tsx
│   │   ├── Select.tsx
│   │   ├── Slider.tsx
│   │   ├── Toggle.tsx
│   │   ├── Card.tsx
│   │   ├── Badge.tsx
│   │   └── ...
│   ├── layout/
│   │   ├── TitleBar.tsx
│   │   ├── Sidebar.tsx
│   │   ├── ContentArea.tsx
│   │   └── Footer.tsx
│   ├── settings/
│   │   ├── GeneralSettings.tsx
│   │   ├── ModelSettings.tsx
│   │   ├── AdvancedSettings.tsx
│   │   └── ...
│   ├── onboarding/
│   │   ├── Welcome.tsx
│   │   ├── Permissions.tsx
│   │   ├── ModelSelect.tsx
│   │   └── ...
│   └── overlay/
│       └── RecordingOverlay.tsx
├── hooks/
│   ├── useSettings.ts
│   └── useRecordingState.ts
├── stores/
│   └── settingsStore.ts
├── lib/
│   ├── types.ts
│   └── utils.ts
└── i18n/
    └── locales/
```

---

## Inspiration & References

- **Linear** (linear.app) - Dark mode excellence, keyboard-first design
- **Raycast** (raycast.com) - Extensions ecosystem, polished micro-interactions
- **Arc** (arc.net) - Sidebar design, tab management
- **Figma** - Component organization, property panels
- **Vercel Dashboard** - Clean data visualization, status indicators
