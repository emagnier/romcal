import SwiftUI

// MARK: - Cella App Entry Point

@main
struct CellaApp: App {
    var body: some Scene {
        WindowGroup {
            LaunchView()
        }
    }
}

// MARK: - Launch View (Silence at Launch)

struct LaunchView: View {
    @State private var showMain = false

    private let liturgicalColor: Color = MockData.today.color.color

    var body: some View {
        ZStack {
            if showMain {
                ContentView()
                    .transition(.opacity)
            } else {
                // Silent launch screen — parchment with a fine cross
                Color.parchment
                    .ignoresSafeArea()
                    .overlay {
                        VStack(spacing: 16) {
                            // Thin cross
                            ZStack {
                                Rectangle()
                                    .fill(liturgicalColor.opacity(0.4))
                                    .frame(width: 1, height: 32)

                                Rectangle()
                                    .fill(liturgicalColor.opacity(0.4))
                                    .frame(width: 18, height: 1)
                                    .offset(y: -6)
                            }
                        }
                    }
            }
        }
        .onAppear {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.8) {
                withAnimation(.easeIn(duration: 0.3)) {
                    showMain = true
                }
            }
        }
    }
}

// MARK: - Preview

#Preview {
    LaunchView()
}
