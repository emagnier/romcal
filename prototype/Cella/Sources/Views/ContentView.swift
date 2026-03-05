import SwiftUI

// MARK: - Main Content View (Tab Bar)

struct ContentView: View {
    @State private var selectedTab = 0

    private let liturgicalColor: Color = MockData.today.color.color

    var body: some View {
        TabView(selection: $selectedTab) {
            TodayView()
                .tabItem {
                    Label("Aujourd'hui", systemImage: "sun.max")
                }
                .tag(0)

            ReadingsView()
                .tabItem {
                    Label("Lectures", systemImage: "book")
                }
                .tag(1)

            HoursListView()
                .tabItem {
                    Label("Heures", systemImage: "bell")
                }
                .tag(2)

            MoreView()
                .tabItem {
                    Label("Plus", systemImage: "plus")
                }
                .tag(3)
        }
        .tint(liturgicalColor)
    }
}

// MARK: - Preview

#Preview {
    ContentView()
}
